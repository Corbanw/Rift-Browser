import 'dart:ffi';
import 'dart:typed_data';
import 'dart:convert';
import 'package:ffi/ffi.dart';

Pointer<Uint8> convertStringToNativeUtf8(String html) {
  try {
    final htmlBytes = Uint8List.fromList(utf8.encode(html));
    final htmlPtr = calloc<Uint8>(htmlBytes.length + 1);
    for (int i = 0; i < htmlBytes.length; i++) {
      htmlPtr[i] = htmlBytes[i];
    }
    htmlPtr[htmlBytes.length] = 0; // Null terminator
    return htmlPtr;
  } catch (e) {
    rethrow;
  }
} 