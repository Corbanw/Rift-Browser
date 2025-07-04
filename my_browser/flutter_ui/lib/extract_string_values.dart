import 'dart:ffi';
import 'ffi_structs.dart';

Map<String, String> extractStringValues(FFILayoutBox ffiBox, String Function(Pointer<Char>, String) safeCStringToString) {
  return {
    'node_type': safeCStringToString(ffiBox.node_type, 'unknown'),
    'text_content': safeCStringToString(ffiBox.text_content, ''),
    'background_color': safeCStringToString(ffiBox.background_color, ''),
    'color': safeCStringToString(ffiBox.color, ''),
    'font_family': safeCStringToString(ffiBox.font_family, ''),
    'border_color': safeCStringToString(ffiBox.border_color, ''),
    'text_align': safeCStringToString(ffiBox.text_align, 'left'),
  };
} 