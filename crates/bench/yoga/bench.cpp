// The Yoga side of souple-bench: the same capture files, the same measured leaves, the same
// scenarios and the same timing loop as crates/bench/src (see README.md), on Yoga built from the
// lecodes-next checkout (read-only).
//
// Kept from Yoga's own harness (benchmark/Benchmark.cpp): the key-by-key style mapping
// (`setStylesFromJson`, its string → enum parsers from benchmark/TreeDeserialization.cpp, compiled
// in as they are), the build order (`buildTreeFromJson`: create, measure function, insert, style,
// children), one config per capture config.
// Changed, and why:
// - the JSON is read once into a plan; the timed build replays the plan's setter calls (Yoga's
//   harness times its JSON lookups and string compares as "tree creation");
// - the measure function does not SLEEP for the recorded duration (on Windows `sleep_for` of a few
//   µs sleeps a timer tick, ~1 ms or more, per call) and does not print on a miss: it is the
//   replay + text model of src/measure.rs (`Spec::answer` = `MeasureSpec::answer`);
// - the root's viewport pin (min == max == the available size) is dropped, as src/plan.rs does;
// - many samples with a warm-up, median / p95, and the incremental scenarios.

#include <algorithm>
#include <chrono>
#include <cmath>
#include <cstdio>
#include <filesystem>
#include <fstream>
#include <map>
#include <string>
#include <vector>

#include <benchmark/TreeDeserialization.h>
#include <nlohmann/json.hpp>
#include <yoga/Yoga.h>
#include <yoga/event/event.h>
#include <yoga/node/Node.h>

using json = nlohmann::json;
using Clock = std::chrono::steady_clock;
namespace fs = std::filesystem;
using namespace facebook::yoga;

#ifndef BENCH_FLAGS
#define BENCH_FLAGS "unknown flags"
#endif

// ── measured leaves: src/measure.rs, line for line ─────────────────────────────────────────────

enum class Mode : uint8_t { Undefined, Exactly, AtMost };

struct Record {
  float width;
  Mode widthMode;
  float height;
  Mode heightMode;
  float outWidth;
  float outHeight;
  double durationNs; // what the call cost when captured: reported, never spent
};

struct Spec {
  std::vector<Record> records;
  float naturalWidth = 10.0f;
  float naturalHeight = 10.0f;
  bool singleLine = false;
  float bump = 0.0f;
  double costNs = 0.0; // the mean captured duration of a call

  void derive() {
    auto widest = [&](bool fixedAllowed) -> const Record* {
      const Record* best = nullptr;
      for (const auto& r : records) {
        if (!fixedAllowed && r.widthMode == Mode::Exactly) continue;
        if (best == nullptr || r.outWidth > best->outWidth) best = &r;
      }
      return best;
    };
    const Record* natural = widest(false);
    if (natural == nullptr) natural = widest(true);
    if (natural != nullptr) {
      naturalWidth = natural->outWidth;
      naturalHeight = natural->outHeight;
    }
    singleLine = false;
    costNs = 0.0;
    for (const auto& r : records) {
      if (r.outWidth < naturalWidth - 0.5f && r.outHeight <= naturalHeight + 0.01f) singleLine = true;
      costNs += r.durationNs;
    }
    if (!records.empty()) costNs /= (double)records.size();
  }

  YGSize answer(float width, Mode widthMode, float height, Mode heightMode) const {
    for (const auto& r : records) {
      if (r.widthMode == widthMode && r.heightMode == heightMode &&
          (widthMode == Mode::Undefined || r.width == width) &&
          (heightMode == Mode::Undefined || r.height == height)) {
        return {r.outWidth, r.outHeight + bump};
      }
    }
    float w = widthMode == Mode::Exactly ? width
        : widthMode == Mode::AtMost      ? std::min(naturalWidth, width)
                                         : naturalWidth;
    float lines = (singleLine || w >= naturalWidth - 0.01f) ? 1.0f : std::ceil(naturalWidth / std::max(w, 1.0f));
    return {w, naturalHeight * lines + bump};
  }
};

static uint64_t gMeasureCalls = 0;
static double gMeasureCostNs = 0.0;
// A diagnostic: every config's errata replaced (`--errata none`); empty = the captures' own.
static std::string gErrataOverride;

static Mode toMode(YGMeasureMode mode) {
  switch (mode) {
    case YGMeasureModeExactly: return Mode::Exactly;
    case YGMeasureModeAtMost: return Mode::AtMost;
    default: return Mode::Undefined;
  }
}

static YGSize measureLeaf(YGNodeConstRef node, float width, YGMeasureMode widthMode, float height, YGMeasureMode heightMode) {
  ++gMeasureCalls;
  auto* spec = static_cast<const Spec*>(YGNodeGetContext(node));
  gMeasureCostNs += spec->costNs;
  return spec->answer(width, toMode(widthMode), height, toMode(heightMode));
}

static Mode modeFromString(const std::string& s) {
  if (s == "exactly") return Mode::Exactly;
  if (s == "at-most") return Mode::AtMost;
  return Mode::Undefined;
}

// ── styles: Benchmark.cpp's setStylesFromJson, as a list of calls ──────────────────────────────

enum class Op : uint8_t {
  FlexDirection, Justify, AlignItems, AlignContent, AlignSelf, Wrap, Overflow, Display, PositionType,
  FlexGrow, FlexShrink, Flex, FlexBasisAuto, FlexBasis, FlexBasisPct,
  Position, PositionPct, Padding, PaddingPct, Border, Margin, MarginPct, MarginAuto, Gap,
  HeightAuto, Height, HeightPct, WidthAuto, Width, WidthPct,
  MinHeight, MinHeightPct, MinWidth, MinWidthPct, MaxHeight, MaxHeightPct, MaxWidth, MaxWidthPct,
};

struct StyleOp {
  Op op;
  int arg; // an enum value, an edge, a gutter
  float value;
};

static std::string edgeOf(const std::string& key, const std::string& property) {
  return key.substr(property.length() + 1);
}

static std::vector<StyleOp> parseStyle(json& style) {
  std::vector<StyleOp> ops;
  if (!style.is_object()) return ops;
  for (auto& [key, value] : style.items()) {
    auto num = [&](json& v) { return v["value"].get<float>(); };
    auto push = [&](Op op, int arg, float v) { ops.push_back({op, arg, v}); };
    if (key == "flex-direction") push(Op::FlexDirection, flexDirectionFromString(value), 0);
    else if (key == "justify-content") push(Op::Justify, justifyContentFromString(value), 0);
    else if (key == "align-items") push(Op::AlignItems, alignFromString(value), 0);
    else if (key == "align-content") push(Op::AlignContent, alignFromString(value), 0);
    else if (key == "align-self") push(Op::AlignSelf, alignFromString(value), 0);
    else if (key == "flex-wrap") push(Op::Wrap, wrapFromString(value), 0);
    else if (key == "overflow") push(Op::Overflow, overflowFromString(value), 0);
    else if (key == "display") push(Op::Display, displayFromString(value), 0);
    else if (key == "position-type") push(Op::PositionType, positionTypeFromString(value), 0);
    else if (key == "flex-grow") push(Op::FlexGrow, 0, value.get<float>());
    else if (key == "flex-shrink") push(Op::FlexShrink, 0, value.get<float>());
    else if (key == "flex") push(Op::Flex, 0, value.get<float>());
    else if (key == "flex-basis") {
      YGUnit unit = unitFromJson(value);
      if (unit == YGUnitAuto) push(Op::FlexBasisAuto, 0, 0);
      else if (unit == YGUnitPoint) push(Op::FlexBasis, 0, num(value));
      else if (unit == YGUnitPercent) push(Op::FlexBasisPct, 0, num(value));
    } else if (key.starts_with("position")) {
      YGEdge edge = edgeFromString(edgeOf(key, "position"));
      YGUnit unit = unitFromJson(value);
      if (unit == YGUnitPoint) push(Op::Position, edge, num(value));
      else if (unit == YGUnitPercent) push(Op::PositionPct, edge, num(value));
    } else if (key.starts_with("padding")) {
      YGEdge edge = edgeFromString(edgeOf(key, "padding"));
      YGUnit unit = unitFromJson(value);
      if (unit == YGUnitPoint) push(Op::Padding, edge, num(value));
      else if (unit == YGUnitPercent) push(Op::PaddingPct, edge, num(value));
    } else if (key.starts_with("border")) {
      YGEdge edge = edgeFromString(edgeOf(key, "border"));
      if (unitFromJson(value) == YGUnitPoint) push(Op::Border, edge, num(value));
    } else if (key.starts_with("margin")) {
      YGEdge edge = edgeFromString(edgeOf(key, "margin"));
      YGUnit unit = unitFromJson(value);
      if (unit == YGUnitPoint) push(Op::Margin, edge, num(value));
      else if (unit == YGUnitPercent) push(Op::MarginPct, edge, num(value));
      else if (unit == YGUnitAuto) push(Op::MarginAuto, edge, 0);
    } else if (key == "gap" || key == "column-gap" || key == "row-gap") {
      if (unitFromJson(value) == YGUnitPoint) {
        push(Op::Gap, key == "gap" ? YGGutterAll : key == "column-gap" ? YGGutterColumn : YGGutterRow, num(value));
      }
    } else if (key == "height" || key == "width") {
      bool h = key == "height";
      YGUnit unit = unitFromJson(value);
      if (unit == YGUnitAuto) push(h ? Op::HeightAuto : Op::WidthAuto, 0, 0);
      else if (unit == YGUnitPoint) push(h ? Op::Height : Op::Width, 0, num(value));
      else if (unit == YGUnitPercent) push(h ? Op::HeightPct : Op::WidthPct, 0, num(value));
    } else if (key == "min-height" || key == "min-width" || key == "max-height" || key == "max-width") {
      YGUnit unit = unitFromJson(value);
      bool pct = unit == YGUnitPercent;
      if (unit != YGUnitPoint && !pct) continue;
      Op op = key == "min-height" ? (pct ? Op::MinHeightPct : Op::MinHeight)
          : key == "min-width"    ? (pct ? Op::MinWidthPct : Op::MinWidth)
          : key == "max-height"   ? (pct ? Op::MaxHeightPct : Op::MaxHeight)
                                  : (pct ? Op::MaxWidthPct : Op::MaxWidth);
      push(op, 0, num(value));
    }
  }
  return ops;
}

static void applyStyle(YGNodeRef n, const std::vector<StyleOp>& ops) {
  for (const auto& o : ops) {
    switch (o.op) {
      case Op::FlexDirection: YGNodeStyleSetFlexDirection(n, YGFlexDirection(o.arg)); break;
      case Op::Justify: YGNodeStyleSetJustifyContent(n, YGJustify(o.arg)); break;
      case Op::AlignItems: YGNodeStyleSetAlignItems(n, YGAlign(o.arg)); break;
      case Op::AlignContent: YGNodeStyleSetAlignContent(n, YGAlign(o.arg)); break;
      case Op::AlignSelf: YGNodeStyleSetAlignSelf(n, YGAlign(o.arg)); break;
      case Op::Wrap: YGNodeStyleSetFlexWrap(n, YGWrap(o.arg)); break;
      case Op::Overflow: YGNodeStyleSetOverflow(n, YGOverflow(o.arg)); break;
      case Op::Display: YGNodeStyleSetDisplay(n, YGDisplay(o.arg)); break;
      case Op::PositionType: YGNodeStyleSetPositionType(n, YGPositionType(o.arg)); break;
      case Op::FlexGrow: YGNodeStyleSetFlexGrow(n, o.value); break;
      case Op::FlexShrink: YGNodeStyleSetFlexShrink(n, o.value); break;
      case Op::Flex: YGNodeStyleSetFlex(n, o.value); break;
      case Op::FlexBasisAuto: YGNodeStyleSetFlexBasisAuto(n); break;
      case Op::FlexBasis: YGNodeStyleSetFlexBasis(n, o.value); break;
      case Op::FlexBasisPct: YGNodeStyleSetFlexBasisPercent(n, o.value); break;
      case Op::Position: YGNodeStyleSetPosition(n, YGEdge(o.arg), o.value); break;
      case Op::PositionPct: YGNodeStyleSetPositionPercent(n, YGEdge(o.arg), o.value); break;
      case Op::Padding: YGNodeStyleSetPadding(n, YGEdge(o.arg), o.value); break;
      case Op::PaddingPct: YGNodeStyleSetPaddingPercent(n, YGEdge(o.arg), o.value); break;
      case Op::Border: YGNodeStyleSetBorder(n, YGEdge(o.arg), o.value); break;
      case Op::Margin: YGNodeStyleSetMargin(n, YGEdge(o.arg), o.value); break;
      case Op::MarginPct: YGNodeStyleSetMarginPercent(n, YGEdge(o.arg), o.value); break;
      case Op::MarginAuto: YGNodeStyleSetMarginAuto(n, YGEdge(o.arg)); break;
      case Op::Gap: YGNodeStyleSetGap(n, YGGutter(o.arg), o.value); break;
      case Op::HeightAuto: YGNodeStyleSetHeightAuto(n); break;
      case Op::Height: YGNodeStyleSetHeight(n, o.value); break;
      case Op::HeightPct: YGNodeStyleSetHeightPercent(n, o.value); break;
      case Op::WidthAuto: YGNodeStyleSetWidthAuto(n); break;
      case Op::Width: YGNodeStyleSetWidth(n, o.value); break;
      case Op::WidthPct: YGNodeStyleSetWidthPercent(n, o.value); break;
      case Op::MinHeight: YGNodeStyleSetMinHeight(n, o.value); break;
      case Op::MinHeightPct: YGNodeStyleSetMinHeightPercent(n, o.value); break;
      case Op::MinWidth: YGNodeStyleSetMinWidth(n, o.value); break;
      case Op::MinWidthPct: YGNodeStyleSetMinWidthPercent(n, o.value); break;
      case Op::MaxHeight: YGNodeStyleSetMaxHeight(n, o.value); break;
      case Op::MaxHeightPct: YGNodeStyleSetMaxHeightPercent(n, o.value); break;
      case Op::MaxWidth: YGNodeStyleSetMaxWidth(n, o.value); break;
      case Op::MaxWidthPct: YGNodeStyleSetMaxWidthPercent(n, o.value); break;
    }
  }
}

// ── the plan ───────────────────────────────────────────────────────────────────────────────────

struct PlanNode {
  int parent = -1;
  int depth = 0;
  std::vector<StyleOp> ops;
  int spec = -1;
  YGConfigRef config = nullptr;
  int children = 0;
  int subtree = 0;
  bool hidden = false;
  bool absolute = false;
};

struct Plan {
  std::string name;
  float width = 0, height = 0;
  std::vector<PlanNode> nodes;
  std::vector<Spec> specs;
  std::map<std::string, YGConfigRef> configs;
  int viewportPin = 0;

  int maxDepth() const {
    int d = 0;
    for (auto& n : nodes) d = std::max(d, n.depth);
    return d;
  }

  // The picks: src/plan.rs, the same rules.
  template <class Pred>
  int deepest(Pred pred) const {
    int best = -1;
    for (int i = 0; i < (int)nodes.size(); i++) {
      if (nodes[i].hidden || !pred(nodes[i])) continue;
      if (best < 0 || nodes[i].depth > nodes[best].depth) best = i;
    }
    return best < 0 ? 0 : best;
  }
  int deepLeaf() const { return deepest([](const PlanNode& n) { return n.children == 0; }); }
  int deepMeasured() const {
    return specs.empty() ? -1 : deepest([](const PlanNode& n) { return n.spec >= 0; });
  }
  int tweenNode() const {
    long long target = std::max<long long>((long long)nodes.size() / 20, 3);
    long long bestDistance = -1;
    int best = 0;
    for (int i = 1; i < (int)nodes.size(); i++) {
      const auto& n = nodes[i];
      if (n.hidden || n.children == 0 || n.subtree < 3 || n.absolute) continue;
      long long distance = std::llabs((long long)n.subtree - target);
      if (bestDistance < 0 || distance < bestDistance) {
        bestDistance = distance;
        best = i;
      }
    }
    return best;
  }
};

static YGConfigRef configFor(Plan& plan, json& config) {
  std::string key = config.dump();
  auto it = plan.configs.find(key);
  if (it != plan.configs.end()) return it->second;
  YGConfigRef c = YGConfigNew();
  if (config.is_object()) {
    for (auto& [k, v] : config.items()) {
      if (k == "use-web-defaults") YGConfigSetUseWebDefaults(c, v.get<bool>());
      else if (k == "point-scale-factor") YGConfigSetPointScaleFactor(c, v.get<float>());
      else if (k == "errata" && gErrataOverride.empty()) YGConfigSetErrata(c, errataFromString(v.get<std::string>()));
      else if (k == "experimental-features") {
        for (auto& f : v) YGConfigSetExperimentalFeatureEnabled(c, experimentalFeatureFromString(f.get<std::string>()), true);
      }
    }
  }
  if (!gErrataOverride.empty()) YGConfigSetErrata(c, errataFromString(gErrataOverride));
  plan.configs[key] = c;
  return c;
}

static Plan loadPlan(const fs::path& path) {
  std::ifstream file(path);
  json capture = json::parse(file);
  Plan plan;
  plan.name = path.stem().string();
  plan.width = capture["layout-inputs"]["available-width"].get<float>();
  plan.height = capture["layout-inputs"]["available-height"].get<float>();

  struct Item { json* j; int parent; int depth; bool hidden; };
  std::vector<Item> stack{{&capture["tree"], -1, 0, false}};
  while (!stack.empty()) {
    Item item = stack.back();
    stack.pop_back();
    json& j = *item.j;
    PlanNode node;
    node.parent = item.parent;
    node.depth = item.depth;
    node.config = configFor(plan, j["config"]);
    node.ops = parseStyle(j["style"]);
    bool none = false;
    for (auto& o : node.ops) {
      if (o.op == Op::Display) none = o.arg == YGDisplayNone;
      if (o.op == Op::PositionType) node.absolute = o.arg == YGPositionTypeAbsolute;
    }
    node.hidden = item.hidden || none;
    if (j.contains("node") && j["node"].is_object() && j["node"].contains("measure-funcs")) {
      Spec spec;
      for (auto& r : j["node"]["measure-funcs"]) {
        auto number = [&](const char* k) { return r[k].is_null() ? NAN : r[k].get<float>(); };
        spec.records.push_back({number("width"), modeFromString(r["width-mode"]), number("height"),
                                modeFromString(r["height-mode"]), number("output-width"), number("output-height"),
                                r.contains("duration-ns") ? r["duration-ns"].get<double>() : 0.0});
      }
      spec.derive();
      plan.specs.push_back(spec);
      node.spec = (int)plan.specs.size() - 1;
    }
    int index = (int)plan.nodes.size();
    if (j.contains("children") && j["children"].is_array()) {
      auto& children = j["children"];
      node.children = (int)children.size();
      for (int c = (int)children.size() - 1; c >= 0; c--) {
        stack.push_back({&children[c], index, item.depth + 1, node.hidden});
      }
    }
    plan.nodes.push_back(std::move(node));
  }
  for (int i = (int)plan.nodes.size() - 1; i >= 0; i--) {
    plan.nodes[i].subtree += 1;
    if (plan.nodes[i].parent >= 0) plan.nodes[plan.nodes[i].parent].subtree += plan.nodes[i].subtree;
  }
  // The viewport pin (src/plan.rs, `load`): min == max == the available size on a root axis.
  auto& root = plan.nodes[0].ops;
  for (int axis = 0; axis < 2; axis++) {
    Op minOp = axis == 0 ? Op::MinWidth : Op::MinHeight;
    Op maxOp = axis == 0 ? Op::MaxWidth : Op::MaxHeight;
    float size = axis == 0 ? plan.width : plan.height;
    // The last write of each wins, as in Yoga.
    const StyleOp* mn = nullptr;
    const StyleOp* mx = nullptr;
    bool otherUnit = false;
    for (auto& o : root) {
      if (o.op == minOp) mn = &o;
      if (o.op == maxOp) mx = &o;
      if (o.op == (axis == 0 ? Op::MinWidthPct : Op::MinHeightPct) || o.op == (axis == 0 ? Op::MaxWidthPct : Op::MaxHeightPct)) otherUnit = true;
    }
    if (mn && mx && !otherUnit && mn->value == size && mx->value == size) {
      root.erase(std::remove_if(root.begin(), root.end(), [&](const StyleOp& o) { return o.op == minOp || o.op == maxOp; }), root.end());
      plan.viewportPin++;
    }
  }
  return plan;
}

// ── a tree built from the plan ─────────────────────────────────────────────────────────────────

struct Built {
  std::vector<YGNodeRef> nodes;
  YGNodeRef root() const { return nodes[0]; }
  void free() {
    if (!nodes.empty()) YGNodeFreeRecursive(nodes[0]);
    nodes.clear();
  }
};

static Built build(Plan& plan) {
  Built b;
  b.nodes.resize(plan.nodes.size());
  for (size_t i = 0; i < plan.nodes.size(); i++) {
    const auto& p = plan.nodes[i];
    YGNodeRef node = YGNodeNewWithConfig(p.config);
    if (p.spec >= 0) {
      YGNodeSetContext(node, &plan.specs[p.spec]);
      YGNodeSetMeasureFunc(node, measureLeaf);
    }
    if (p.parent >= 0) {
      YGNodeRef parent = b.nodes[p.parent];
      YGNodeInsertChild(parent, node, YGNodeGetChildCount(parent));
    }
    applyStyle(node, p.ops);
    b.nodes[i] = node;
  }
  return b;
}

// ── timing: src/harness.rs ─────────────────────────────────────────────────────────────────────

constexpr size_t kFloorSamples = 3;

struct Budget {
  double seconds = 0.5;
  size_t minSamples = 30;
  size_t maxSamples = 20000;
  // src/harness.rs, `Budget::wants`.
  bool wants(size_t taken, double elapsedUs) const {
    bool capped = taken >= kFloorSamples && elapsedUs > 10.0 * seconds * 1e6;
    return (taken < minSamples && !capped) || (elapsedUs < seconds * 1e6 && taken < maxSamples);
  }
};

struct Stats {
  double median = 0, p95 = 0, mean = 0, min = 0;
  size_t samples = 0, batch = 1;
  std::vector<std::pair<std::string, double>> counters;

  static Stats from(std::vector<double> v, size_t batch) {
    std::sort(v.begin(), v.end());
    auto at = [&](double q) { return v[(size_t)std::llround((double)(v.size() - 1) * q)]; };
    Stats s;
    s.median = at(0.5);
    s.p95 = at(0.95);
    double sum = 0;
    for (double x : v) sum += x;
    s.mean = sum / (double)v.size();
    s.min = v[0];
    s.samples = v.size();
    s.batch = batch;
    return s;
  }

  json toJson() const {
    json j = {{"median", median}, {"p95", p95}, {"mean", mean}, {"min", min}, {"samples", samples}, {"batch", batch}};
    json c = json::object();
    for (auto& [k, v] : counters) c[k] = v;
    j["counters"] = c;
    return j;
  }
};

static double us(Clock::time_point a, Clock::time_point b) {
  return std::chrono::duration<double, std::micro>(b - a).count();
}

constexpr double kMinSampleUs = 5.0;

template <class F>
static Stats bench(const Budget& budget, F op) {
  auto start = Clock::now();
  uint64_t calls = 0;
  while (calls < 3 || (us(start, Clock::now()) < 20000.0 && calls < 10000000)) {
    op();
    calls++;
  }
  double perCall = us(start, Clock::now()) / (double)calls;
  size_t batch = perCall >= kMinSampleUs ? 1 : std::min<size_t>((size_t)std::ceil(kMinSampleUs / perCall), 100000);
  std::vector<double> samples;
  auto begin = Clock::now();
  while (budget.wants(samples.size(), us(begin, Clock::now()))) {
    auto t = Clock::now();
    for (size_t i = 0; i < batch; i++) op();
    samples.push_back(us(t, Clock::now()) / (double)batch);
  }
  return Stats::from(std::move(samples), batch);
}

struct Sampler {
  Clock::time_point begin = Clock::now();
  size_t taken = 0;
  Budget budget;
  explicit Sampler(Budget b) : budget(b) {}
  bool more() {
    bool m = budget.wants(taken, us(begin, Clock::now()));
    taken++;
    return m;
  }
};

// Yoga's own per-pass counters (LayoutData), summed over the passes run while counting.
struct YogaCounts {
  double layouts = 0, measures = 0, cachedLayouts = 0, cachedMeasures = 0;
};
static YogaCounts gCounts;

static void startCounting() {
  gCounts = {};
  gMeasureCalls = 0;
  gMeasureCostNs = 0.0;
  Event::subscribe([](YGNodeConstRef, Event::Type type, Event::Data data) {
    if (type == Event::LayoutPassEnd) {
      auto* d = data.get<Event::LayoutPassEnd>().layoutData;
      gCounts.layouts += d->layouts;
      gCounts.measures += d->measures;
      gCounts.cachedLayouts += d->cachedLayouts;
      gCounts.cachedMeasures += d->cachedMeasures;
    }
  });
}

static void stopCounting(Stats& s, double passes) {
  Event::reset();
  s.counters.push_back({"measure_calls", (double)gMeasureCalls / passes});
  s.counters.push_back({"measure_us_captured", gMeasureCostNs / 1e3 / passes});
  s.counters.push_back({"layouts", gCounts.layouts / passes});
  s.counters.push_back({"measures", gCounts.measures / passes});
  s.counters.push_back({"cached_layouts", gCounts.cachedLayouts / passes});
  s.counters.push_back({"cached_measures", gCounts.cachedMeasures / passes});
}

constexpr int kCountPasses = 20;
constexpr float kTweenFrom = 200.0f;
constexpr int kTweenFrames = 60;
constexpr long long kResizeAmplitude = 32;

static float resizeOffset(uint64_t i) {
  long long a = kResizeAmplitude;
  long long p = (long long)(i % (uint64_t)(4 * a));
  return (float)(p < 2 * a ? p - a : 3 * a - p);
}

static void restoreWidth(YGNodeRef node, YGValue width) {
  switch (width.unit) {
    case YGUnitPoint: YGNodeStyleSetWidth(node, width.value); break;
    case YGUnitPercent: YGNodeStyleSetWidthPercent(node, width.value); break;
    case YGUnitAuto: YGNodeStyleSetWidthAuto(node); break;
    default: YGNodeStyleSetWidth(node, YGUndefined); break;
  }
}

// ── the scenarios: src/scenarios.rs ────────────────────────────────────────────────────────────

static json runTree(Plan& plan, const Budget& budget, const fs::path* rects) {
  json scenarios = json::object();
  const float W = plan.width, H = plan.height;
  auto layout = [&](YGNodeRef root, float w, float h) { YGNodeCalculateLayout(root, w, h, YGDirectionLTR); };

  // Full layout of a fresh tree.
  {
    for (int i = 0; i < 2; i++) {
      Built b = build(plan);
      layout(b.root(), W, H);
      b.free();
    }
    std::vector<double> tBuild, tLayout, tTotal;
    Sampler sampler(budget);
    while (sampler.more()) {
      auto t0 = Clock::now();
      Built b = build(plan);
      auto t1 = Clock::now();
      layout(b.root(), W, H);
      auto t2 = Clock::now();
      tBuild.push_back(us(t0, t1));
      tLayout.push_back(us(t1, t2));
      tTotal.push_back(us(t0, t2));
      b.free();
    }
    scenarios["build"] = Stats::from(tBuild, 1).toJson();
    Stats l = Stats::from(tLayout, 1);
    // Yoga's counters for one full layout, from one more (untimed) tree.
    Built b = build(plan);
    startCounting();
    layout(b.root(), W, H);
    stopCounting(l, 1);
    b.free();
    scenarios["layout"] = l.toJson();
    scenarios["build_layout"] = Stats::from(tTotal, 1).toJson();
  }

  Built b = build(plan);
  YGNodeRef root = b.root();
  layout(root, W, H);
  if (rects) {
    std::ofstream out(*rects / (plan.name + ".yoga.txt"));
    char line[128];
    for (auto* n : b.nodes) {
      std::snprintf(line, sizeof line, "%.4f %.4f %.4f %.4f\n", YGNodeLayoutGetLeft(n), YGNodeLayoutGetTop(n),
                    YGNodeLayoutGetWidth(n), YGNodeLayoutGetHeight(n));
      out << line;
    }
  }

  auto steadyOver = [&](const char* name, int passes, auto pass) {
    Stats s = bench(budget, pass);
    startCounting();
    for (int i = 0; i < passes; i++) pass();
    stopCounting(s, passes);
    scenarios[name] = s.toJson();
  };
  auto steady = [&](const char* name, auto pass) { steadyOver(name, kCountPasses, pass); };

  steady("noop", [&] { layout(root, W, H); });

  {
    int index = plan.deepLeaf();
    YGNodeRef leaf = b.nodes[index];
    YGValue original = YGNodeStyleGetWidth(leaf);
    YGNodeStyleSetWidth(leaf, 10.0f);
    layout(root, W, H);
    bool flip = false;
    steady("leaf_resize", [&] {
      flip = !flip;
      YGNodeStyleSetWidth(leaf, flip ? 11.0f : 10.0f);
      layout(root, W, H);
    });
    restoreWidth(leaf, original);
    layout(root, W, H);
  }

  if (int index = plan.deepMeasured(); index >= 0) {
    YGNodeRef leaf = b.nodes[index];
    Spec& spec = plan.specs[plan.nodes[index].spec];
    bool flip = false;
    steady("leaf_dirty", [&] {
      flip = !flip;
      spec.bump = flip ? 1.0f : 0.0f;
      YGNodeMarkDirty(leaf);
      layout(root, W, H);
    });
    spec.bump = 0.0f;
    YGNodeMarkDirty(leaf);
    layout(root, W, H);
  }

  {
    // Counted over one whole period of the triangle: what a pass finds in the caches depends on
    // where in the wave it is (src/scenarios.rs, VIEWPORT_PASSES).
    uint64_t i = 0;
    steadyOver("viewport", int(4 * kResizeAmplitude), [&] {
      i++;
      layout(root, W + resizeOffset(i), H);
    });
    layout(root, W, H);
  }

  {
    int index = plan.tweenNode();
    YGNodeRef node = b.nodes[index];
    YGValue original = YGNodeStyleGetWidth(node);
    YGNodeStyleSetWidth(node, kTweenFrom);
    layout(root, W, H);
    std::vector<double> passes, runs;
    Budget tb = budget;
    tb.minSamples = 10;
    tb.maxSamples = budget.maxSamples / kTweenFrames;
    Sampler sampler(tb);
    size_t run = 0;
    bool warm = true;
    while (warm || sampler.more()) {
      auto runStart = Clock::now();
      for (int frame = 1; frame <= kTweenFrames; frame++) {
        float width = run % 2 == 0 ? kTweenFrom + (float)frame : kTweenFrom + (float)(kTweenFrames - frame);
        auto t = Clock::now();
        YGNodeStyleSetWidth(node, width);
        layout(root, W, H);
        if (!warm) passes.push_back(us(t, Clock::now()));
      }
      if (!warm) runs.push_back(us(runStart, Clock::now()));
      warm = false;
      run++;
    }
    Stats s = Stats::from(passes, 1);
    // One more run, untimed, counted (a subscriber to Yoga's events costs time).
    startCounting();
    for (int frame = 1; frame <= kTweenFrames; frame++) {
      float width = run % 2 == 0 ? kTweenFrom + (float)frame : kTweenFrom + (float)(kTweenFrames - frame);
      YGNodeStyleSetWidth(node, width);
      layout(root, W, H);
    }
    stopCounting(s, kTweenFrames);
    scenarios["tween"] = s.toJson();
    scenarios["tween_60"] = Stats::from(runs, 1).toJson();
    restoreWidth(node, original);
    layout(root, W, H);
  }

  b.free();

  int measured = plan.deepMeasured();
  json picks = {{"leaf", plan.deepLeaf()}, {"measured", measured < 0 ? json(nullptr) : json(measured)}, {"tween", plan.tweenNode()}};
  return {
      {"name", plan.name},
      {"nodes", plan.nodes.size()},
      {"measured", plan.specs.size()},
      {"depth", plan.maxDepth()},
      {"picks", picks},
      {"notes", {{"viewport_pin", plan.viewportPin}, {"configs", plan.configs.size()}}},
      {"scenarios", scenarios},
  };
}

static std::string compilerName() {
#if defined(__clang__)
  return std::string("clang ") + __clang_version__;
#elif defined(_MSC_VER)
  return "MSVC " + std::to_string(_MSC_FULL_VER);
#elif defined(__GNUC__)
  return std::string("gcc ") + __VERSION__;
#else
  return "unknown compiler";
#endif
}

int main(int argc, char** argv) {
  Budget budget;
  fs::path out = "yoga.json";
  fs::path rectsDir;
  bool haveRects = false;
  std::vector<fs::path> inputs;
  for (int i = 1; i < argc; i++) {
    std::string a = argv[i];
    if (a == "--seconds" && i + 1 < argc) budget.seconds = std::stod(argv[++i]);
    else if (a == "--min-samples" && i + 1 < argc) budget.minSamples = std::stoul(argv[++i]);
    else if (a == "--out" && i + 1 < argc) out = argv[++i];
    else if (a == "--rects" && i + 1 < argc) { rectsDir = argv[++i]; haveRects = true; }
    else if (a == "--errata" && i + 1 < argc) gErrataOverride = argv[++i];
    else inputs.push_back(a);
  }
  std::vector<fs::path> files;
  for (auto& p : inputs) {
    if (fs::is_directory(p)) {
      std::vector<fs::path> list;
      for (auto& e : fs::directory_iterator(p))
        if (e.path().extension() == ".json") list.push_back(e.path());
      std::sort(list.begin(), list.end());
      files.insert(files.end(), list.begin(), list.end());
    } else {
      files.push_back(p);
    }
  }
  if (haveRects) fs::create_directories(rectsDir);

  json trees = json::array();
  for (auto& f : files) {
    Plan plan = loadPlan(f);
    std::fprintf(stderr, "%s: %zu nodes, %zu measured, depth %d\n", plan.name.c_str(), plan.nodes.size(), plan.specs.size(), plan.maxDepth());
    json t = runTree(plan, budget, haveRects ? &rectsDir : nullptr);
    for (auto& [k, v] : t["scenarios"].items()) {
      std::fprintf(stderr, "  %-16s median %10.2f us  p95 %10.2f us  (%zux%zu)\n", k.c_str(), v["median"].get<double>(),
                   v["p95"].get<double>(), v["samples"].get<size_t>(), v["batch"].get<size_t>());
    }
    trees.push_back(t);
    for (auto& [k, c] : plan.configs) YGConfigFree(c);
  }
  const char* machine = std::getenv("BENCH_MACHINE");
  json result = {
      {"engine", "yoga"},
      {"machine", machine ? machine : "unknown machine"},
      {"build", compilerName() + ", " + BENCH_FLAGS + (gErrataOverride.empty() ? "" : ", errata overridden: " + gErrataOverride)},
      {"budget_seconds", budget.seconds},
      {"sizes", {{"yoga::Node", sizeof(facebook::yoga::Node)}, {"yoga::Style", sizeof(facebook::yoga::Style)}, {"yoga::LayoutResults", sizeof(facebook::yoga::LayoutResults)}}},
      {"trees", trees},
  };
  std::ofstream(out) << result.dump(2);
  std::fprintf(stderr, "wrote %s\n", out.string().c_str());
  return 0;
}
