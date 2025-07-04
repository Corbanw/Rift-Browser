import 'dart:ffi';
import 'ffi_types.dart';

Pointer<Void> callParseHtml(ParseHtmlDart parseHtml, Pointer<Uint8> htmlPtr) {
  try {
    final boxArrayPtr = parseHtml(htmlPtr.cast<Char>());
    return boxArrayPtr;
  } catch (e) {
    return Pointer<Void>.fromAddress(0);
  }
} 