import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';
import 'models/layout_box.dart';
import 'dev_console.dart';

class DOMRenderer extends CustomPainter {
  final List<LayoutBox> layoutBoxes;
  final double scrollOffset;
  final Size viewportSize;
  final int maxBoxesToRender;
  final bool isRenderingInProgress;
  final double renderingProgress;
  final String renderingMessage;

  DOMRenderer({
    required this.layoutBoxes,
    this.scrollOffset = 0.0,
    this.viewportSize = const Size(800, 600),
    this.maxBoxesToRender = 25,
    this.isRenderingInProgress = false,
    this.renderingProgress = 0.0,
    this.renderingMessage = '',
  });

  // Constants
  static const double _viewportPadding = 100.0;
  static const double _minFontSize = 8.0;
  static const int _maxTextLength = 200;
  static const int _maxTextLines = 3;
  static const double _defaultMaxWidth = 200.0;
// Render 10 boxes per batch

  @override
  void paint(Canvas canvas, Size size) {
    try {
      final stopwatch = Stopwatch()..start();
      logPrint('[RENDER] Starting paint with ${layoutBoxes.length} boxes, viewport: ${size.width}x${size.height}');
      
      // Draw rendering progress overlay if rendering is in progress
      if (isRenderingInProgress) {
        _drawRenderingProgressOverlay(canvas, size);
      }
      
      final visibleBoxes = _getVisibleBoxes(size);
      logPrint('[RENDER] Filtered to ${visibleBoxes.length} visible boxes out of ${layoutBoxes.length} total');
      
      _drawBackgrounds(canvas, visibleBoxes, size);
      _drawTexts(canvas, visibleBoxes, size);
      
      stopwatch.stop();
      logPrint('[RENDER] Paint completed in ${stopwatch.elapsedMilliseconds}ms');
    } catch (e) {
      logPrint('[RENDER] Paint error: $e');
      _drawErrorFallback(canvas, size, e.toString());
    }
  }

  void _drawRenderingProgressOverlay(Canvas canvas, Size size) {
    try {
      // Semi-transparent overlay
      final overlayPaint = Paint()
        ..color = Colors.black.withOpacity(0.3)
        ..style = PaintingStyle.fill;
      canvas.drawRect(Rect.fromLTWH(0, 0, size.width, size.height), overlayPaint);
      
      // Progress indicator
      final centerX = size.width / 2;
      final centerY = size.height / 2;
      
      // Progress circle
      final progressPaint = Paint()
        ..color = Colors.blue
        ..style = PaintingStyle.stroke
        ..strokeWidth = 4.0;
      
      const radius = 30.0;
      canvas.drawCircle(Offset(centerX, centerY), radius, progressPaint);
      
      // Progress arc
      final progressArcPaint = Paint()
        ..color = Colors.white
        ..style = PaintingStyle.stroke
        ..strokeWidth = 4.0
        ..strokeCap = StrokeCap.round;
      
      final sweepAngle = 2 * 3.14159 * renderingProgress;
      canvas.drawArc(
        Rect.fromCircle(center: Offset(centerX, centerY), radius: radius),
        -3.14159 / 2, // Start from top
        sweepAngle,
        false,
        progressArcPaint,
      );
      
      // Progress text
      final textPainter = TextPainter(
        text: TextSpan(
          text: '${(renderingProgress * 100).toInt()}%',
          style: const TextStyle(
            color: Colors.white,
            fontSize: 16,
            fontWeight: FontWeight.bold,
          ),
        ),
        textDirection: TextDirection.ltr,
      );
      textPainter.layout();
      textPainter.paint(
        canvas,
        Offset(centerX - textPainter.width / 2, centerY - textPainter.height / 2),
      );
      
      // Message text
      if (renderingMessage.isNotEmpty) {
        final messagePainter = TextPainter(
          text: TextSpan(
            text: renderingMessage,
            style: const TextStyle(
              color: Colors.white,
              fontSize: 14,
            ),
          ),
          textDirection: TextDirection.ltr,
          textAlign: TextAlign.center,
        );
        messagePainter.layout(maxWidth: size.width - 40);
        messagePainter.paint(
          canvas,
          Offset(20, centerY + 50),
        );
      }
    } catch (e) {
      logPrint('[RENDER] Error drawing progress overlay: $e');
    }
  }

  void _drawErrorFallback(Canvas canvas, Size size, String error) {
    try {
      final paint = Paint()
        ..color = Colors.red
        ..style = PaintingStyle.fill;
      
      canvas.drawRect(Rect.fromLTWH(0, 0, size.width, size.height), paint);
      
      final textPainter = TextPainter(
        text: TextSpan(
          text: 'Rendering Error: $error',
          style: const TextStyle(color: Colors.white, fontSize: 14),
        ),
        textDirection: TextDirection.ltr,
      );
      
      textPainter.layout();
      textPainter.paint(canvas, const Offset(10, 10));
    } catch (e) {
      logPrint('[RENDER] Error fallback failed: $e');
    }
  }

  List<LayoutBox> _getVisibleBoxes(Size size) {
    try {
      final visibleRect = _calculateVisibleRect(size);
      final expandedRect = visibleRect.inflate(_viewportPadding);
      
      final visibleBoxes = <LayoutBox>[];
      int processedCount = 0;
      
      for (final box in layoutBoxes) {
        processedCount++;
        if (processedCount > maxBoxesToRender) {
          logPrint('[RENDER] Reached max boxes limit ($maxBoxesToRender), stopping');
          break;
        }
        
        final boxRect = Rect.fromLTWH(box.x, box.y, box.width, box.height);
        if (expandedRect.overlaps(boxRect)) {
          visibleBoxes.add(box);
        }
      }
      
      return visibleBoxes;
    } catch (e) {
      logPrint('[RENDER] _getVisibleBoxes error: $e');
      return [];
    }
  }

  Rect _calculateVisibleRect(Size size) {
    return Rect.fromLTWH(0, scrollOffset, size.width, size.height);
  }

  void _drawBackgrounds(Canvas canvas, List<LayoutBox> visibleBoxes, Size size) {
    try {
      final paint = Paint();
      int backgroundCount = 0;
      
      for (final box in visibleBoxes) {
        try {
          if (box.nodeType == 'text') continue;
          
          final adjustedY = box.y - scrollOffset;
          if (!_isBoxInViewport(adjustedY, box.height, size.height)) continue;
          
          // Draw background
          if (_shouldDrawBackground(box)) {
            paint.color = _parseColor(box.backgroundColor);
            paint.style = PaintingStyle.fill;
            
            // Apply margin to background
            final backgroundRect = Rect.fromLTWH(
              box.x + box.margin,
              adjustedY + box.margin,
              box.width - (box.margin * 2),
              box.height - (box.margin * 2),
            );
            
            canvas.drawRect(backgroundRect, paint);
            backgroundCount++;
          }
          
          // Draw border
          if (_shouldDrawBorder(box)) {
            paint.color = _parseColor(box.borderColor);
            paint.style = PaintingStyle.stroke;
            paint.strokeWidth = box.borderWidth;
            
            final borderRect = Rect.fromLTWH(
              box.x + box.margin,
              adjustedY + box.margin,
              box.width - (box.margin * 2),
              box.height - (box.margin * 2),
            );
            
            canvas.drawRect(borderRect, paint);
          }
        } catch (e) {
          logPrint('[RENDER] Error drawing background for box: $e');
          continue;
        }
      }
      
      logPrint('[RENDER] Drew $backgroundCount backgrounds');
    } catch (e) {
      logPrint('[RENDER] _drawBackgrounds error: $e');
    }
  }

  bool _isBoxInViewport(double adjustedY, double height, double viewportHeight) {
    return adjustedY + height >= 0 && adjustedY <= viewportHeight;
  }

  bool _shouldDrawBackground(LayoutBox box) {
    return box.backgroundColor.isNotEmpty && 
           box.backgroundColor != 'transparent' && 
           box.backgroundColor != 'rgba(0,0,0,0)';
  }

  bool _shouldDrawBorder(LayoutBox box) {
    return box.borderColor.isNotEmpty && 
           box.borderColor != 'transparent' && 
           box.borderWidth > 0;
  }

  void _drawTexts(Canvas canvas, List<LayoutBox> visibleBoxes, Size size) {
    try {
      int textCount = 0;
      
      for (final box in visibleBoxes) {
        try {
          if (box.nodeType != 'text') continue;
          
          final adjustedY = box.y - scrollOffset;
          if (!_isBoxInViewport(adjustedY, box.height, size.height)) continue;
          
          if (_shouldDrawText(box)) {
            _drawSingleText(canvas, box, adjustedY);
            textCount++;
          }
        } catch (e) {
          logPrint('[RENDER] Error painting text "${box.textContent}": $e');
          continue;
        }
      }
      
      logPrint('[RENDER] Drew $textCount texts');
    } catch (e) {
      logPrint('[RENDER] _drawTexts error: $e');
    }
  }

  bool _shouldDrawText(LayoutBox box) {
    return box.textContent.isNotEmpty && 
           box.textContent.trim().isNotEmpty &&
           box.fontSize >= _minFontSize && 
           box.textContent.length <= _maxTextLength;
  }

  void _drawSingleText(Canvas canvas, LayoutBox box, double adjustedY) {
    try {
      final textPainter = _createTextPainter(box);
      final maxWidth = box.width > 0 ? box.width : _defaultMaxWidth;
      
      // Handle word wrapping
      final shouldWrap = box.wordWrap.toLowerCase() == 'break-word' || 
                        box.wordWrap.toLowerCase() == 'break-all';
      
      textPainter.layout(maxWidth: shouldWrap ? maxWidth : double.infinity);
      
      // Handle text overflow
      String displayText = box.textContent;
      if (box.textOverflow.toLowerCase() == 'ellipsis' && 
          textPainter.didExceedMaxLines) {
        displayText = '${box.textContent}...';
      }
      
      // Create new text painter with potentially modified text
      final finalTextPainter = TextPainter(
        text: TextSpan(
          text: displayText,
          style: _createTextStyle(box),
        ),
        textDirection: TextDirection.ltr,
        textAlign: _mapTextAlign(box.textAlign),
        maxLines: _maxTextLines,
      );
      
      finalTextPainter.layout(maxWidth: shouldWrap ? maxWidth : double.infinity);
      
      finalTextPainter.paint(
        canvas,
        Offset(box.x + box.padding, adjustedY + box.padding),
      );
    } catch (e) {
      logPrint('[RENDER] _drawSingleText error: $e');
    }
  }

  TextPainter _createTextPainter(LayoutBox box) {
    try {
      return TextPainter(
        text: TextSpan(
          text: box.textContent,
          style: _createTextStyle(box),
        ),
        textDirection: TextDirection.ltr,
        textAlign: _mapTextAlign(box.textAlign),
        maxLines: _maxTextLines,
      );
    } catch (e) {
      logPrint('[RENDER] _createTextPainter error: $e');
      return TextPainter(
        text: const TextSpan(text: 'Error', style: TextStyle(color: Colors.red)),
        textDirection: TextDirection.ltr,
      );
    }
  }

  TextStyle _createTextStyle(LayoutBox box) {
    try {
      // Handle dark mode
      final isDarkMode = box.colorScheme.toLowerCase() == 'dark';
      final defaultColor = isDarkMode ? Colors.white : Colors.black;
      
      return TextStyle(
        color: _parseColor(box.color.isEmpty ? defaultColor.toString() : box.color),
        fontSize: box.fontSize,
        fontFamily: _getFontFamily(box.fontFamily),
        fontWeight: _mapFontWeight(box.fontWeight),
        height: box.lineHeight > 0 ? box.lineHeight : null,
        overflow: _mapTextOverflow(box.textOverflow),
      );
    } catch (e) {
      logPrint('[RENDER] _createTextStyle error: $e');
      return const TextStyle(color: Colors.black, fontSize: 12);
    }
  }

  String? _getFontFamily(String fontFamily) {
    return fontFamily.isNotEmpty ? fontFamily : null;
  }

  @override
  bool shouldRepaint(DOMRenderer oldDelegate) {
    return oldDelegate.layoutBoxes != layoutBoxes ||
           oldDelegate.scrollOffset != scrollOffset ||
           oldDelegate.viewportSize != viewportSize ||
           oldDelegate.maxBoxesToRender != maxBoxesToRender ||
           oldDelegate.isRenderingInProgress != isRenderingInProgress ||
           oldDelegate.renderingProgress != renderingProgress ||
           oldDelegate.renderingMessage != renderingMessage;
  }

  Color _parseColor(String colorString) {
    try {
      if (colorString.isEmpty) return Colors.black;
      
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
      logPrint('[RENDER] Error parsing color "$colorString": $e');
      return Colors.black;
    }
  }

  Color _parseHexColor(String colorString) {
    try {
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
    } catch (e) {
      logPrint('[RENDER] _parseHexColor error: $e');
      return Colors.black;
    }
  }

  Color _parseRgbColor(String colorString) {
    try {
      final values = colorString
          .substring(4, colorString.length - 1)
          .split(',')
          .map((s) => int.parse(s.trim()))
          .toList();
      return Color.fromRGBO(values[0], values[1], values[2], 1.0);
    } catch (e) {
      logPrint('[RENDER] _parseRgbColor error: $e');
      return Colors.black;
    }
  }

  Color _parseRgbaColor(String colorString) {
    try {
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
    } catch (e) {
      logPrint('[RENDER] _parseRgbaColor error: $e');
      return Colors.black;
    }
  }

  Color _parseNamedColor(String colorString) {
    switch (colorString.toLowerCase()) {
      case 'black':
        return Colors.black;
      case 'white':
        return Colors.white;
      case 'red':
        return Colors.red;
      case 'green':
        return Colors.green;
      case 'blue':
        return Colors.blue;
      case 'yellow':
        return Colors.yellow;
      case 'gray':
      case 'grey':
        return Colors.grey;
      case 'transparent':
        return Colors.transparent;
      default:
        return Colors.black;
    }
  }

  FontWeight _mapFontWeight(double weight) {
    if (weight >= 700) return FontWeight.bold;
    if (weight >= 600) return FontWeight.w600;
    if (weight >= 500) return FontWeight.w500;
    if (weight >= 400) return FontWeight.normal;
    if (weight >= 300) return FontWeight.w300;
    if (weight >= 200) return FontWeight.w200;
    return FontWeight.w100;
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

  void logPrint(Object? obj) {
    // Use the same logPrint function from main.dart
    // ignore: avoid_print
    print(obj);
  }
}

// New batched renderer widget
class BatchedDOMRenderer extends StatefulWidget {
  final List<LayoutBox> layoutBoxes;
  final double scrollOffset;
  final Size viewportSize;
  final int maxBoxesToRender;

  const BatchedDOMRenderer({
    super.key,
    required this.layoutBoxes,
    this.scrollOffset = 0.0,
    this.viewportSize = const Size(800, 600),
    this.maxBoxesToRender = 25,
  });

  @override
  State<BatchedDOMRenderer> createState() => _BatchedDOMRendererState();
}

class _BatchedDOMRendererState extends State<BatchedDOMRenderer> {
  List<LayoutBox> _renderedBoxes = [];
  bool _isRendering = false;
  double _renderingProgress = 0.0;
  String _renderingMessage = '';
  int _currentBatchIndex = 0;
  static const int _batchSize = 5; // Reduced from 10 to 5 for more conservative rendering

  @override
  void initState() {
    super.initState();
    _startBatchedRendering();
  }

  @override
  void didUpdateWidget(BatchedDOMRenderer oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.layoutBoxes != widget.layoutBoxes) {
      _startBatchedRendering();
    }
  }

  void _startBatchedRendering() {
    if (_isRendering) return;
    
    setState(() {
      _isRendering = true;
      _renderingProgress = 0.0;
      _renderingMessage = 'Starting render...';
      _currentBatchIndex = 0;
      _renderedBoxes = [];
    });

    _renderNextBatch();
  }

  void _renderNextBatch() {
    if (!mounted) return;

    final totalBoxes = widget.layoutBoxes.length;
    final startIndex = _currentBatchIndex * _batchSize;
    final endIndex = (startIndex + _batchSize).clamp(0, totalBoxes);
    
    if (startIndex >= totalBoxes) {
      _finishRendering();
      return;
    }

    try {
      // Add this batch to rendered boxes
      final newBatch = widget.layoutBoxes.sublist(startIndex, endIndex);
      setState(() {
        _renderedBoxes.addAll(newBatch);
        _currentBatchIndex++;
        _renderingProgress = _renderedBoxes.length / totalBoxes;
        _renderingMessage = 'Rendering batch $_currentBatchIndex/${(totalBoxes / _batchSize).ceil()}';
      });

      // Schedule next batch with error handling
      SchedulerBinding.instance.addPostFrameCallback((_) {
        try {
          _renderNextBatch();
        } catch (e) {
          logPrint('[RENDER] Error in _renderNextBatch: $e');
          _finishRendering();
        }
      });
    } catch (e) {
      logPrint('[RENDER] Error processing batch: $e');
      _finishRendering();
    }
  }

  void _finishRendering() {
    if (!mounted) return;
    
    setState(() {
      _isRendering = false;
      _renderingProgress = 1.0;
      _renderingMessage = 'Render complete';
    });

    // Log completion
    LogManager().add('[RENDER] Batched rendering completed: ${_renderedBoxes.length} boxes');
  }

  @override
  Widget build(BuildContext context) {
    return CustomPaint(
      painter: DOMRenderer(
        layoutBoxes: _renderedBoxes,
        scrollOffset: widget.scrollOffset,
        viewportSize: widget.viewportSize,
        maxBoxesToRender: widget.maxBoxesToRender,
        isRenderingInProgress: _isRendering,
        renderingProgress: _renderingProgress,
        renderingMessage: _renderingMessage,
      ),
      size: _calculateContentSize(),
    );
  }

  Size _calculateContentSize() {
    try {
      if (_renderedBoxes.isEmpty) {
        return widget.viewportSize;
      }
      
      final maxY = _renderedBoxes.map((box) => box.y + box.height).reduce((a, b) => a > b ? a : b);
      return Size(widget.viewportSize.width, maxY + 100);
    } catch (e) {
      logPrint('[RENDER] _calculateContentSize: Error: $e');
      return widget.viewportSize;
    }
  }

  void logPrint(Object? obj) {
    // ignore: avoid_print
    print(obj);
  }
} 