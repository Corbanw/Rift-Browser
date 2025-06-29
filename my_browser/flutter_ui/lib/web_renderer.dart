import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'models/layout_box.dart';
import 'dev_console.dart';

void logPrint(Object? obj) {
  LogManager().add(obj?.toString() ?? 'null');
  // Optionally, also print to console
  // ignore: avoid_print
  print(obj);
}

/// A dynamic web renderer that builds Flutter widgets from DOM and CSS data
class WebRenderer extends StatelessWidget {
  final List<LayoutBox> layoutBoxes;
  final double scrollOffset;
  final Size viewportSize;
  final bool isDarkMode;
  final int maxElementsToRender;

  const WebRenderer({
    super.key,
    required this.layoutBoxes,
    this.scrollOffset = 0.0,
    this.viewportSize = const Size(800, 600),
    this.isDarkMode = false,
    this.maxElementsToRender = 100,
  });

  @override
  Widget build(BuildContext context) {
    try {
      // Safety check for null or empty layout boxes
      if (layoutBoxes.isEmpty) {
        logPrint('[WEB_RENDERER] No layout boxes to render');
        return const Center(
          child: Text('No content to display'),
        );
      }
      
      logPrint('[WEB_RENDERER] Building web renderer with ${layoutBoxes.length} elements');
      
      // Convert layout boxes to renderable elements
      final renderableElements = _convertToRenderableElements();
      
      // Build the widget tree
      return _buildWidgetTree(renderableElements);
      
    } catch (e) {
      logPrint('[WEB_RENDERER] Error building renderer: $e');
      return _buildErrorWidget(e.toString());
    }
  }

  List<RenderableElement> _convertToRenderableElements() {
    final elements = <RenderableElement>[];
    
    try {
      // Limit the number of elements to prevent crashes
      final maxElements = maxElementsToRender.clamp(1, 1000);
      final elementsToProcess = layoutBoxes.length.clamp(0, maxElements);
      
      logPrint('[WEB_RENDERER] Processing $elementsToProcess out of ${layoutBoxes.length} layout boxes');
      
      for (int i = 0; i < elementsToProcess; i++) {
        final box = layoutBoxes[i];
        try {
          final element = RenderableElement.fromLayoutBox(box, isDarkMode);
          elements.add(element);
        } catch (e) {
          logPrint('[WEB_RENDERER] Error converting box $i: $e');
          // Continue processing other boxes instead of crashing
          continue;
        }
      }
    } catch (e) {
      logPrint('[WEB_RENDERER] Critical error in _convertToRenderableElements: $e');
      // Return empty list instead of crashing
      return [];
    }
    
    logPrint('[WEB_RENDERER] Converted ${elements.length} elements');
    return elements;
  }

  Widget _buildWidgetTree(List<RenderableElement> elements) {
    if (elements.isEmpty) {
      return const Center(
        child: Text('No content to render'),
      );
    }

    // For large numbers of elements, use a more efficient approach
    if (elements.length > 100) {
      return _buildOptimizedWidgetTree(elements);
    }

    // Group elements by their container relationships
    final containers = _groupIntoContainers(elements);
    
    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: containers.map((container) => _buildContainer(container)).toList(),
      ),
    );
  }

  Widget _buildOptimizedWidgetTree(List<RenderableElement> elements) {
    try {
      // For large pages, use ListView.builder for better performance
      return ListView.builder(
        itemCount: elements.length,
        itemBuilder: (context, index) {
          try {
            if (index >= elements.length) {
              logPrint('[WEB_RENDERER] Index out of bounds: $index >= ${elements.length}');
              return const SizedBox.shrink();
            }
            
            final element = elements[index];
            return _buildOptimizedElement(element);
          } catch (e) {
            logPrint('[WEB_RENDERER] Error building element $index: $e');
            return const SizedBox.shrink();
          }
        },
      );
    } catch (e) {
      logPrint('[WEB_RENDERER] Critical error in _buildOptimizedWidgetTree: $e');
      return _buildErrorWidget('Failed to build widget tree: $e');
    }
  }

  Widget _buildOptimizedElement(RenderableElement element) {
    // Simplified rendering for performance
    return Container(
      width: element.width > 0 ? element.width : null,
      height: element.height > 0 ? element.height : null,
      margin: EdgeInsets.only(
        left: element.x,
        top: element.y,
      ),
      padding: EdgeInsets.all(element.padding),
      decoration: BoxDecoration(
        color: element.backgroundColor,
        border: element.borderWidth > 0 
            ? Border.all(color: element.borderColor, width: element.borderWidth)
            : null,
      ),
      child: element.textContent.isNotEmpty
          ? Text(
              element.textContent,
              style: TextStyle(
                color: element.textColor,
                fontSize: element.fontSize,
                fontWeight: element.fontWeight >= 700 ? FontWeight.bold : FontWeight.normal,
              ),
            )
          : null,
    );
  }

  List<List<RenderableElement>> _groupIntoContainers(List<RenderableElement> elements) {
    final containers = <List<RenderableElement>>[];
    final processed = <int>{};
    
    for (int i = 0; i < elements.length; i++) {
      if (processed.contains(i)) continue;
      
      final container = <RenderableElement>[];
      final element = elements[i];
      
      // Check if this is a container element
      if (_isContainerElement(element)) {
        container.add(element);
        processed.add(i);
        
        // Find children of this container
        for (int j = i + 1; j < elements.length; j++) {
          if (processed.contains(j)) continue;
          
          final child = elements[j];
          if (_isChildOf(element, child)) {
            container.add(child);
            processed.add(j);
          }
        }
      } else {
        // Standalone element
        container.add(element);
        processed.add(i);
      }
      
      containers.add(container);
    }
    
    return containers;
  }

  bool _isContainerElement(RenderableElement element) {
    return element.tagName == 'div' || 
           element.tagName == 'section' || 
           element.tagName == 'article' ||
           element.tagName == 'main' ||
           element.tagName == 'header' ||
           element.tagName == 'footer' ||
           element.tagName == 'nav';
  }

  bool _isChildOf(RenderableElement parent, RenderableElement child) {
    // Simple heuristic: if child is positioned within parent's bounds
    return child.y >= parent.y && 
           child.y < parent.y + parent.height &&
           child.x >= parent.x && 
           child.x < parent.x + parent.width;
  }

  Widget _buildContainer(List<RenderableElement> containerElements) {
    if (containerElements.isEmpty) return const SizedBox.shrink();
    
    final container = containerElements.first;
    final children = containerElements.skip(1).toList();
    
    Widget containerWidget;
    
    // Build container based on display type
    switch (container.displayType) {
      case DisplayType.flex:
        containerWidget = _buildFlexContainer(container, children);
        break;
      case DisplayType.grid:
        containerWidget = _buildGridContainer(container, children);
        break;
      case DisplayType.block:
      default:
        containerWidget = _buildBlockContainer(container, children);
        break;
    }
    
    return containerWidget;
  }

  Widget _buildFlexContainer(RenderableElement container, List<RenderableElement> children) {
    final direction = container.flexDirection == 'column' 
        ? Axis.vertical 
        : Axis.horizontal;
    
    final mainAxisAlignment = _mapMainAxisAlignment(container.justifyContent);
    final crossAxisAlignment = _mapCrossAxisAlignment(container.alignItems);
    
    return Container(
      width: container.width > 0 ? container.width : null,
      height: container.height > 0 ? container.height : null,
      margin: EdgeInsets.all(container.margin),
      padding: EdgeInsets.all(container.padding),
      decoration: _buildDecoration(container),
      child: Flex(
        direction: direction,
        mainAxisAlignment: mainAxisAlignment,
        crossAxisAlignment: crossAxisAlignment,
        children: children.map((child) => _buildFlexChild(child)).toList(),
      ),
    );
  }

  Widget _buildGridContainer(RenderableElement container, List<RenderableElement> children) {
    // Simple 2-column grid for now
    const columns = 2;
    final rows = (children.length / columns).ceil();
    
    return Container(
      width: container.width > 0 ? container.width : null,
      height: container.height > 0 ? container.height : null,
      margin: EdgeInsets.all(container.margin),
      padding: EdgeInsets.all(container.padding),
      decoration: _buildDecoration(container),
      child: GridView.count(
        shrinkWrap: true,
        physics: const NeverScrollableScrollPhysics(),
        crossAxisCount: columns,
        mainAxisSpacing: 10,
        crossAxisSpacing: 10,
        children: children.map((child) => _buildElement(child)).toList(),
      ),
    );
  }

  Widget _buildBlockContainer(RenderableElement container, List<RenderableElement> children) {
    return Container(
      width: container.width > 0 ? container.width : null,
      height: container.height > 0 ? container.height : null,
      margin: EdgeInsets.all(container.margin),
      padding: EdgeInsets.all(container.padding),
      decoration: _buildDecoration(container),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: children.map((child) => _buildElement(child)).toList(),
      ),
    );
  }

  Widget _buildFlexChild(RenderableElement element) {
    return Flexible(
      flex: element.flexGrow.toInt(),
      child: _buildElement(element),
    );
  }

  Widget _buildElement(RenderableElement element) {
    switch (element.tagName) {
      case 'h1':
      case 'h2':
      case 'h3':
      case 'h4':
      case 'h5':
      case 'h6':
        return _buildHeading(element);
      case 'p':
        return _buildParagraph(element);
      case 'a':
        return _buildLink(element);
      case 'button':
        return _buildButton(element);
      case 'img':
        return _buildImage(element);
      case 'text':
        return _buildText(element);
      default:
        return _buildGenericElement(element);
    }
  }

  Widget _buildHeading(RenderableElement element) {
    final fontSize = _getHeadingFontSize(element.tagName);
    
    return Container(
      width: element.width > 0 ? element.width : null,
      margin: EdgeInsets.all(element.margin),
      child: Text(
        element.textContent,
        style: TextStyle(
          fontSize: fontSize,
          fontWeight: FontWeight.bold,
          color: element.textColor,
          fontFamily: element.fontFamily.isNotEmpty ? element.fontFamily : null,
        ),
        textAlign: _mapTextAlign(element.textAlign),
      ),
    );
  }

  Widget _buildParagraph(RenderableElement element) {
    return Container(
      width: element.width > 0 ? element.width : null,
      margin: EdgeInsets.all(element.margin),
      child: Text(
        element.textContent,
        style: TextStyle(
          fontSize: element.fontSize,
          fontWeight: _mapFontWeight(element.fontWeight),
          color: element.textColor,
          fontFamily: element.fontFamily.isNotEmpty ? element.fontFamily : null,
          height: element.lineHeight > 0 ? element.lineHeight : null,
        ),
        textAlign: _mapTextAlign(element.textAlign),
        overflow: _mapTextOverflow(element.textOverflow),
        maxLines: element.textOverflow == 'ellipsis' ? 1 : null,
      ),
    );
  }

  Widget _buildLink(RenderableElement element) {
    return Container(
      margin: EdgeInsets.all(element.margin),
      child: InkWell(
        onTap: () {
          logPrint('[WEB_RENDERER] Link clicked: ${element.textContent}');
        },
        child: Text(
          element.textContent,
          style: TextStyle(
            fontSize: element.fontSize,
            color: Colors.blue,
            decoration: TextDecoration.underline,
            fontFamily: element.fontFamily.isNotEmpty ? element.fontFamily : null,
          ),
        ),
      ),
    );
  }

  Widget _buildButton(RenderableElement element) {
    return Container(
      margin: EdgeInsets.all(element.margin),
      child: ElevatedButton(
        onPressed: () {
          logPrint('[WEB_RENDERER] Button clicked: ${element.textContent}');
        },
        style: ElevatedButton.styleFrom(
          backgroundColor: element.backgroundColor,
          foregroundColor: element.textColor,
        ),
        child: Text(element.textContent),
      ),
    );
  }

  Widget _buildImage(RenderableElement element) {
    return Container(
      width: element.width > 0 ? element.width : null,
      height: element.height > 0 ? element.height : null,
      margin: EdgeInsets.all(element.margin),
      decoration: BoxDecoration(
        color: Colors.grey[300],
        border: Border.all(color: Colors.grey[400]!),
      ),
      child: const Center(
        child: Icon(Icons.image, size: 48, color: Colors.grey),
      ),
    );
  }

  Widget _buildText(RenderableElement element) {
    return Container(
      margin: EdgeInsets.all(element.margin),
      child: Text(
        element.textContent,
        style: TextStyle(
          fontSize: element.fontSize,
          fontWeight: _mapFontWeight(element.fontWeight),
          color: element.textColor,
          fontFamily: element.fontFamily.isNotEmpty ? element.fontFamily : null,
        ),
      ),
    );
  }

  Widget _buildGenericElement(RenderableElement element) {
    return Container(
      width: element.width > 0 ? element.width : null,
      height: element.height > 0 ? element.height : null,
      margin: EdgeInsets.all(element.margin),
      padding: EdgeInsets.all(element.padding),
      decoration: _buildDecoration(element),
      child: element.textContent.isNotEmpty 
          ? Text(
              element.textContent,
              style: TextStyle(
                fontSize: element.fontSize,
                fontWeight: _mapFontWeight(element.fontWeight),
                color: element.textColor,
                fontFamily: element.fontFamily.isNotEmpty ? element.fontFamily : null,
              ),
            )
          : null,
    );
  }

  BoxDecoration _buildDecoration(RenderableElement element) {
    return BoxDecoration(
      color: element.backgroundColor,
      border: element.borderWidth > 0 
          ? Border.all(
              color: element.borderColor,
              width: element.borderWidth,
            )
          : null,
      borderRadius: BorderRadius.circular(4),
    );
  }

  double _getHeadingFontSize(String tagName) {
    switch (tagName) {
      case 'h1': return 32;
      case 'h2': return 24;
      case 'h3': return 20;
      case 'h4': return 18;
      case 'h5': return 16;
      case 'h6': return 14;
      default: return 16;
    }
  }

  MainAxisAlignment _mapMainAxisAlignment(String alignment) {
    switch (alignment.toLowerCase()) {
      case 'center':
        return MainAxisAlignment.center;
      case 'flex-end':
      case 'end':
        return MainAxisAlignment.end;
      case 'space-between':
        return MainAxisAlignment.spaceBetween;
      case 'space-around':
        return MainAxisAlignment.spaceAround;
      case 'space-evenly':
        return MainAxisAlignment.spaceEvenly;
      case 'flex-start':
      case 'start':
      default:
        return MainAxisAlignment.start;
    }
  }

  CrossAxisAlignment _mapCrossAxisAlignment(String alignment) {
    switch (alignment.toLowerCase()) {
      case 'center':
        return CrossAxisAlignment.center;
      case 'flex-end':
      case 'end':
        return CrossAxisAlignment.end;
      case 'stretch':
        return CrossAxisAlignment.stretch;
      case 'flex-start':
      case 'start':
      default:
        return CrossAxisAlignment.start;
    }
  }

  TextAlign _mapTextAlign(String align) {
    switch (align.toLowerCase()) {
      case 'center':
        return TextAlign.center;
      case 'right':
        return TextAlign.right;
      case 'justify':
        return TextAlign.justify;
      case 'left':
      default:
        return TextAlign.left;
    }
  }

  FontWeight _mapFontWeight(double weight) {
    if (weight >= 700) return FontWeight.bold;
    if (weight >= 600) return FontWeight.w600;
    if (weight >= 500) return FontWeight.w500;
    if (weight >= 400) return FontWeight.normal;
    if (weight >= 300) return FontWeight.w300;
    return FontWeight.normal;
  }

  TextOverflow _mapTextOverflow(String overflow) {
    switch (overflow.toLowerCase()) {
      case 'ellipsis':
        return TextOverflow.ellipsis;
      case 'clip':
        return TextOverflow.clip;
      case 'fade':
        return TextOverflow.fade;
      default:
        return TextOverflow.clip;
    }
  }

  Widget _buildErrorWidget(String error) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(Icons.error_outline, size: 64, color: Colors.red),
          const SizedBox(height: 16),
          const Text(
            'Rendering Error',
            style: TextStyle(fontSize: 20, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 8),
          Text(
            'Failed to render page: $error',
            style: const TextStyle(fontSize: 14, color: Colors.grey),
            textAlign: TextAlign.center,
          ),
        ],
      ),
    );
  }
}

/// Represents a renderable element with all its styling and layout properties
class RenderableElement {
  final String tagName;
  final String textContent;
  final double x;
  final double y;
  final double width;
  final double height;
  final Color backgroundColor;
  final Color textColor;
  final double fontSize;
  final String fontFamily;
  final double fontWeight;
  final String textAlign;
  final double margin;
  final double padding;
  final double borderWidth;
  final Color borderColor;
  final DisplayType displayType;
  final String flexDirection;
  final String justifyContent;
  final String alignItems;
  final double flexGrow;
  final double lineHeight;
  final String textOverflow;

  RenderableElement({
    required this.tagName,
    required this.textContent,
    required this.x,
    required this.y,
    required this.width,
    required this.height,
    required this.backgroundColor,
    required this.textColor,
    required this.fontSize,
    required this.fontFamily,
    required this.fontWeight,
    required this.textAlign,
    required this.margin,
    required this.padding,
    required this.borderWidth,
    required this.borderColor,
    required this.displayType,
    required this.flexDirection,
    required this.justifyContent,
    required this.alignItems,
    required this.flexGrow,
    required this.lineHeight,
    required this.textOverflow,
  });

  factory RenderableElement.fromLayoutBox(LayoutBox box, bool isDarkMode) {
    final defaultTextColor = isDarkMode ? Colors.white : Colors.black;
    final defaultBgColor = isDarkMode ? Colors.grey[900]! : Colors.transparent;

    return RenderableElement(
      tagName: box.nodeType,
      textContent: box.textContent,
      x: box.x,
      y: box.y,
      width: box.width,
      height: box.height,
      backgroundColor: _parseColor(box.backgroundColor, defaultBgColor),
      textColor: _parseColor(box.color, defaultTextColor),
      fontSize: box.fontSize,
      fontFamily: box.fontFamily,
      fontWeight: box.fontWeight,
      textAlign: box.textAlign,
      margin: box.margin,
      padding: box.padding,
      borderWidth: box.borderWidth,
      borderColor: _parseColor(box.borderColor, Colors.grey),
      displayType: _parseDisplayType(box.flexDirection),
      flexDirection: box.flexDirection,
      justifyContent: box.justifyContent,
      alignItems: box.alignItems,
      flexGrow: box.flexGrow,
      lineHeight: box.lineHeight,
      textOverflow: box.textOverflow,
    );
  }

  static Color _parseColor(String colorString, Color defaultColor) {
    try {
      if (colorString.isEmpty) return defaultColor;
      
      if (colorString.startsWith('#')) {
        return _parseHexColor(colorString);
      } else if (colorString.startsWith('rgb(')) {
        return _parseRgbColor(colorString);
      } else if (colorString.startsWith('rgba(')) {
        return _parseRgbaColor(colorString);
      } else {
        return _parseNamedColor(colorString);
      }
    } catch (e) {
      logPrint('[WEB_RENDERER] Error parsing color "$colorString": $e');
      return defaultColor;
    }
  }

  static Color _parseHexColor(String colorString) {
    final hex = colorString.substring(1);
    if (hex.length == 6) {
      return Color(int.parse('FF$hex', radix: 16));
    } else if (hex.length == 3) {
      final r = hex[0] + hex[0];
      final g = hex[1] + hex[1];
      final b = hex[2] + hex[2];
      return Color(int.parse('FF$r$g$b', radix: 16));
    }
    return Colors.black;
  }

  static Color _parseRgbColor(String colorString) {
    final values = colorString
        .substring(4, colorString.length - 1)
        .split(',')
        .map((s) => int.parse(s.trim()))
        .toList();
    return Color.fromRGBO(values[0], values[1], values[2], 1.0);
  }

  static Color _parseRgbaColor(String colorString) {
    final values = colorString
        .substring(5, colorString.length - 1)
        .split(',')
        .map((s) => double.parse(s.trim()))
        .toList();
    return Color.fromRGBO(
      values[0].toInt(), 
      values[1].toInt(), 
      values[2].toInt(), 
      values[3]
    );
  }

  static Color _parseNamedColor(String colorName) {
    switch (colorName.toLowerCase()) {
      case 'red': return Colors.red;
      case 'green': return Colors.green;
      case 'blue': return Colors.blue;
      case 'yellow': return Colors.yellow;
      case 'orange': return Colors.orange;
      case 'purple': return Colors.purple;
      case 'pink': return Colors.pink;
      case 'brown': return Colors.brown;
      case 'grey':
      case 'gray': return Colors.grey;
      case 'black': return Colors.black;
      case 'white': return Colors.white;
      case 'transparent': return Colors.transparent;
      default: return Colors.black;
    }
  }

  static DisplayType _parseDisplayType(String flexDirection) {
    if (flexDirection.isNotEmpty) return DisplayType.flex;
    return DisplayType.block;
  }

  /// Creates a smart fallback renderer that shows meaningful content
  static List<RenderableElement> createSmartFallback(String url, List<LayoutBox> boxes) {
    final elements = <RenderableElement>[];
    
    if (boxes.isEmpty) {
      // No boxes - create informative fallback
      elements.add(RenderableElement(
        tagName: 'div',
        textContent: 'No content rendered\nURL: $url\nTry a different page or check the URL',
        x: 50.0,
        y: 50.0,
        width: 700.0,
        height: 100.0,
        backgroundColor: Colors.orange.shade100,
        textColor: Colors.orange.shade800,
        fontSize: 14.0,
        fontFamily: 'Arial',
        fontWeight: 400.0,
        textAlign: 'left',
        margin: 10.0,
        padding: 15.0,
        borderWidth: 2.0,
        borderColor: Colors.orange.shade300,
        displayType: DisplayType.block,
        flexDirection: 'column',
        justifyContent: 'flex-start',
        alignItems: 'stretch',
        flexGrow: 0.0,
        lineHeight: 1.4,
        textOverflow: 'ellipsis',
      ));
      return elements;
    }
    
    if (boxes.length <= 5) {
      // Very few boxes - enhance them with better content
      for (int i = 0; i < boxes.length; i++) {
        final box = boxes[i];
        final enhancedText = _enhanceBoxContent(box, i);
        
        elements.add(RenderableElement(
          tagName: box.nodeType,
          textContent: enhancedText,
          x: box.x,
          y: box.y,
          width: box.width,
          height: box.height,
          backgroundColor: _parseColor(box.backgroundColor, Colors.blue.shade50),
          textColor: _parseColor(box.color, Colors.black87),
          fontSize: box.fontSize,
          fontFamily: box.fontFamily,
          fontWeight: box.fontWeight,
          textAlign: box.textAlign,
          margin: box.margin,
          padding: box.padding,
          borderWidth: box.borderWidth,
          borderColor: _parseColor(box.borderColor, Colors.grey.shade400),
          displayType: _parseDisplayType(box.flexDirection),
          flexDirection: box.flexDirection,
          justifyContent: box.justifyContent,
          alignItems: box.alignItems,
          flexGrow: box.flexGrow,
          lineHeight: box.lineHeight,
          textOverflow: box.textOverflow,
        ));
      }
      
      // Add info box about the limited rendering
      elements.add(RenderableElement(
        tagName: 'info',
        textContent: 'Limited rendering: ${boxes.length} elements found\nThis page may need more complex CSS or JavaScript support',
        x: 50.0,
        y: 50.0 + (boxes.length * 80.0),
        width: 700.0,
        height: 60.0,
        backgroundColor: Colors.amber.shade50,
        textColor: Colors.amber.shade800,
        fontSize: 12.0,
        fontFamily: 'Arial',
        fontWeight: 400.0,
        textAlign: 'left',
        margin: 10.0,
        padding: 10.0,
        borderWidth: 1.0,
        borderColor: Colors.amber.shade200,
        displayType: DisplayType.block,
        flexDirection: 'column',
        justifyContent: 'flex-start',
        alignItems: 'stretch',
        flexGrow: 0.0,
        lineHeight: 1.3,
        textOverflow: 'ellipsis',
      ));
      
      return elements;
    }
    
    // Normal rendering for sufficient boxes
    return boxes.map((box) => RenderableElement.fromLayoutBox(box, false)).toList();
  }
  
  /// Enhances box content with more meaningful information
  static String _enhanceBoxContent(LayoutBox box, int index) {
    final tagName = box.nodeType;
    final textContent = box.textContent.trim();
    
    if (textContent.isNotEmpty) {
      return '<$tagName> $textContent';
    }
    
    // Provide meaningful content based on tag type
    switch (tagName.toLowerCase()) {
      case 'div':
        return '<div> Container element #$index';
      case 'p':
        return '<p> Paragraph element #$index';
      case 'span':
        return '<span> Inline element #$index';
      case 'a':
        return '<a> Link element #$index';
      case 'h1':
      case 'h2':
      case 'h3':
      case 'h4':
      case 'h5':
      case 'h6':
        return '<$tagName> Heading element #$index';
      case 'img':
        return '<img> Image element #$index';
      case 'button':
        return '<button> Button element #$index';
      case 'input':
        return '<input> Input element #$index';
      case 'form':
        return '<form> Form element #$index';
      case 'nav':
        return '<nav> Navigation element #$index';
      case 'header':
        return '<header> Header element #$index';
      case 'footer':
        return '<footer> Footer element #$index';
      case 'main':
        return '<main> Main content #$index';
      case 'section':
        return '<section> Section element #$index';
      case 'article':
        return '<article> Article element #$index';
      case 'aside':
        return '<aside> Sidebar element #$index';
      case 'ul':
      case 'ol':
        return '<$tagName> List element #$index';
      case 'li':
        return '<li> List item #$index';
      case 'table':
        return '<table> Table element #$index';
      case 'tr':
        return '<tr> Table row #$index';
      case 'td':
      case 'th':
        return '<$tagName> Table cell #$index';
      default:
        return '<$tagName> Element #$index';
    }
  }
}

enum DisplayType {
  block,
  flex,
  grid,
  inline,
} 