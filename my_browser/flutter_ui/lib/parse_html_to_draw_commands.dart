import 'models/layout_box.dart';
import 'engine_bridge.dart';

DrawCommandResult parseHtmlToDrawCommands(String html, String css) {
  return EngineBridge.parseHtmlToDrawCommands(html, css);
} 