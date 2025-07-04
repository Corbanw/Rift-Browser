import 'models/layout_box.dart';
import 'engine_bridge.dart';

List<LayoutBox> createLayoutFromHtml(String html) {
  return EngineBridge.createLayoutFromHtml(html);
} 