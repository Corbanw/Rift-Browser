import 'models/layout_box.dart';
import 'dart:ffi';
import 'ffi_structs.dart';
import 'package:ffi/ffi.dart';
import 'engine_bridge.dart' show DrawCommand;

String _ffiStringToDart(Pointer<Char> ptr) {
  if (ptr == nullptr) return '';
  final str = ptr.cast<Utf8>().toDartString();
  // Optionally free the string if ownership is transferred
  // calloc.free(ptr);
  return str;
}

int _parseColor(String colorStr) {
  // Accepts hex color strings like '#RRGGBB' or '#AARRGGBB', returns int
  if (colorStr.isEmpty) return 0xFF000000;
  var hex = colorStr.replaceFirst('#', '');
  if (hex.length == 6) hex = 'FF$hex';
  return int.tryParse(hex, radix: 16) ?? 0xFF000000;
}

DrawCommand extractSingleDrawCommand(Pointer<FFIDrawCommand> commandPtr) {
  final cmd = commandPtr.ref;
  switch (cmd.command_type) {
    case 0: // rect
      return DrawCommand.rect(
        x: cmd.x,
        y: cmd.y,
        w: cmd.width,
        h: cmd.height,
        color: _parseColor(_ffiStringToDart(cmd.color)),
      );
    case 1: // text
      return DrawCommand.text(
        x: cmd.x,
        y: cmd.y,
        content: _ffiStringToDart(cmd.text),
        font: '', // font not present in FFI struct, set as empty or extend FFI struct
        size: cmd.font_size,
        color: _parseColor(_ffiStringToDart(cmd.color)),
      );
    case 3: // image
      return DrawCommand.image(
        x: cmd.x,
        y: cmd.y,
        src: _ffiStringToDart(cmd.text), // assuming text field holds image src
      );
    default:
      // Unknown command, return a rect as fallback
      return DrawCommand.rect(
        x: cmd.x,
        y: cmd.y,
        w: cmd.width,
        h: cmd.height,
        color: 0xFF000000,
      );
  }
} 