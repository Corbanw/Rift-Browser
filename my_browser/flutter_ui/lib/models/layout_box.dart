class LayoutBox {
  final double x;
  final double y;
  final double width;
  final double height;
  final String nodeType;
  final String textContent;
  final String backgroundColor;
  final String color;
  final double fontSize;
  final String fontFamily;
  final double borderWidth;
  final String borderColor;
  final double padding;
  final double margin;
  final double fontWeight;
  final String textAlign;
  // Flexbox properties
  final String flexDirection;
  final String flexWrap;
  final String justifyContent;
  final String alignItems;
  final double flexGrow;
  final double flexShrink;
  final String flexBasis;
  final int order;
  // Grid properties
  final String gridColumn;
  final String gridRow;
  // Text rendering
  final double lineHeight;
  final String wordWrap;
  final String whiteSpace;
  final String textOverflow;
  // Theme support
  final String colorScheme;

  LayoutBox({
    required this.x,
    required this.y,
    required this.width,
    required this.height,
    required this.nodeType,
    required this.textContent,
    required this.backgroundColor,
    required this.color,
    required this.fontSize,
    required this.fontFamily,
    required this.borderWidth,
    required this.borderColor,
    required this.padding,
    required this.margin,
    required this.fontWeight,
    required this.textAlign,
    // Flexbox properties
    this.flexDirection = '',
    this.flexWrap = '',
    this.justifyContent = '',
    this.alignItems = '',
    this.flexGrow = 0.0,
    this.flexShrink = 1.0,
    this.flexBasis = '',
    this.order = 0,
    // Grid properties
    this.gridColumn = '',
    this.gridRow = '',
    // Text rendering
    this.lineHeight = 1.2,
    this.wordWrap = '',
    this.whiteSpace = '',
    this.textOverflow = '',
    // Theme support
    this.colorScheme = '',
  });

  @override
  String toString() {
    return 'LayoutBox(x: $x, y: $y, width: $width, height: $height, nodeType: $nodeType, textContent: $textContent, fontWeight: $fontWeight, textAlign: $textAlign)';
  }
} 