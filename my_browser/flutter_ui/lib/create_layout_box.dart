import 'models/layout_box.dart';

LayoutBox createLayoutBox(
  Map<String, double> numericValues,
  Map<String, String> stringValues,
  Map<String, double> calculatedValues,
) {
  return LayoutBox(
    x: numericValues['x'] ?? 0.0,
    y: numericValues['y'] ?? 0.0,
    width: numericValues['width'] ?? 0.0,
    height: numericValues['height'] ?? 0.0,
    nodeType: stringValues['node_type'] ?? '',
    textContent: stringValues['text_content'] ?? '',
    backgroundColor: stringValues['background_color'] ?? '',
    color: stringValues['color'] ?? '',
    fontSize: numericValues['font_size'] ?? 12.0,
    fontFamily: stringValues['font_family'] ?? '',
    borderWidth: calculatedValues['border_width'] ?? 0.0,
    borderColor: stringValues['border_color'] ?? '',
    padding: calculatedValues['padding'] ?? 0.0,
    margin: calculatedValues['margin'] ?? 0.0,
    fontWeight: numericValues['font_weight'] ?? 400.0,
    textAlign: stringValues['text_align'] ?? '',
  );
} 