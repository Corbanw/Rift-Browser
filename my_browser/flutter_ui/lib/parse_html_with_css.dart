import 'models/layout_box.dart';
import 'engine_bridge.dart';

List<LayoutBox> parseHtmlWithCss(String html, String css) {
  return EngineBridge.parseHtmlWithCss(html, css);
} 