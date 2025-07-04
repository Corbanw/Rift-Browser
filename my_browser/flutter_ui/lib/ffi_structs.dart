import 'dart:ffi' as ffi;

final class FFILayoutBox extends ffi.Struct {
  @ffi.Double()
  external double x;
  @ffi.Double()
  external double y;
  @ffi.Double()
  external double width;
  @ffi.Double()
  external double height;
  @ffi.Double()
  external double font_size;
  @ffi.Double()
  external double font_weight;
  external ffi.Pointer<ffi.Char> node_type;
  external ffi.Pointer<ffi.Char> text_content;
  external ffi.Pointer<ffi.Char> background_color;
  external ffi.Pointer<ffi.Char> color;
  external ffi.Pointer<ffi.Char> font_family;
  external ffi.Pointer<ffi.Char> border_color;
  external ffi.Pointer<ffi.Char> text_align;
  @ffi.Double()
  external double margin_top;
  @ffi.Double()
  external double margin_right;
  @ffi.Double()
  external double margin_bottom;
  @ffi.Double()
  external double margin_left;
  @ffi.Double()
  external double padding_top;
  @ffi.Double()
  external double padding_right;
  @ffi.Double()
  external double padding_bottom;
  @ffi.Double()
  external double padding_left;
  @ffi.Double()
  external double border_width_top;
  @ffi.Double()
  external double border_width_right;
  @ffi.Double()
  external double border_width_bottom;
  @ffi.Double()
  external double border_width_left;
}

final class FFIDrawCommand extends ffi.Struct {
  @ffi.Int32()
  external int command_type; // 0=rect, 1=text, 2=line, 3=image
  @ffi.Float()
  external double x;
  @ffi.Float()
  external double y;
  @ffi.Float()
  external double width;
  @ffi.Float()
  external double height;
  external ffi.Pointer<ffi.Char> color;
  external ffi.Pointer<ffi.Char> text;
  @ffi.Float()
  external double font_size;
  @ffi.Float()
  external double font_weight;
}

final class FFIDrawCommandArray extends ffi.Struct {
  external ffi.Pointer<ffi.Pointer<FFIDrawCommand>> commands;
  @ffi.Int32()
  external int total_count;
  @ffi.Int32()
  external int batch_size;
} 