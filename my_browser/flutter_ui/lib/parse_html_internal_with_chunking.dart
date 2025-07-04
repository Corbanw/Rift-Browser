import 'models/layout_box.dart';
import 'dart:ffi';
import 'engine_bridge.dart';

Future<List<LayoutBox>> parseHtmlInternalWithChunking(String html) async {
  return await EngineBridge.parseHtmlInternalWithChunking(html);
} 