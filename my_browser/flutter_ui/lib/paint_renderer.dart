import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'dart:ui' as ui;
import 'dart:typed_data';
import 'engine_bridge.dart';
import 'dev_console.dart';

void logPrint(Object? obj) {
  LogManager().add(obj?.toString() ?? 'null');
  print(obj);
}

/// A renderer that paints HTML and CSS directly to a canvas
class PaintRenderer extends StatefulWidget {
  final String html;
  final String css;
  final double scrollOffset;
  final Size viewportSize;
  final bool isDarkMode;
  final VoidCallback? onRenderComplete;

  const PaintRenderer({
    super.key,
    required this.html,
    required this.css,
    this.scrollOffset = 0.0,
    this.viewportSize = const Size(800, 600),
    this.isDarkMode = false,
    this.onRenderComplete,
  });

  @override
  State<PaintRenderer> createState() => _PaintRendererState();
}

class _PaintRendererState extends State<PaintRenderer> {
  ui.Image? _renderedImage;
  bool _isRendering = false;
  String? _errorMessage;

  @override
  void initState() {
    super.initState();
    _renderContent();
  }

  @override
  void didUpdateWidget(PaintRenderer oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.html != widget.html || 
        oldWidget.css != widget.css ||
        oldWidget.viewportSize != widget.viewportSize ||
        oldWidget.isDarkMode != widget.isDarkMode) {
      _renderContent();
    }
  }

  Future<void> _renderContent() async {
    if (_isRendering) return;

    setState(() {
      _isRendering = true;
      _errorMessage = null;
    });

    try {
      logPrint('[PAINT_RENDERER] Starting render for ${widget.html.length} chars of HTML');
      
      // Get draw commands from the Rust engine
      final drawCommands = await _getDrawCommands();
      
      if (drawCommands.isEmpty) {
        setState(() {
          _errorMessage = 'No content to render';
          _isRendering = false;
        });
        return;
      }

      // Create image from draw commands
      final image = await _createImageFromCommands(drawCommands);
      
      setState(() {
        _renderedImage = image;
        _isRendering = false;
      });

      widget.onRenderComplete?.call();
      
      logPrint('[PAINT_RENDERER] Render completed successfully');
    } catch (e, stack) {
      logPrint('[PAINT_RENDERER] Render error: $e\n$stack');
      setState(() {
        _errorMessage = 'Render error: $e';
        _isRendering = false;
      });
    }
  }

  Future<List<DrawCommand>> _getDrawCommands() async {
    try {
      // Use the Rust engine to parse HTML and get draw commands
      final result = EngineBridge.parseHtmlToDrawCommands(widget.html, widget.css);
      
      if (!result.success) {
        throw Exception('Failed to parse HTML: ${result.errorMessage}');
      }

      return result.drawCommands;
    } catch (e) {
      logPrint('[PAINT_RENDERER] Error getting draw commands: $e');
      rethrow;
    }
  }

  Future<ui.Image> _createImageFromCommands(List<DrawCommand> commands) async {
    final recorder = ui.PictureRecorder();
    final canvas = Canvas(recorder);
    
    // Set up canvas with viewport size
    canvas.clipRect(Rect.fromLTWH(0, 0, widget.viewportSize.width, widget.viewportSize.height));
    
    // Apply scroll offset
    canvas.translate(0, -widget.scrollOffset);
    
    // Draw background
    _drawBackground(canvas);
    
    // Draw all commands
    for (final command in commands) {
      _drawCommand(canvas, command);
    }
    
    final picture = recorder.endRecording();
    return await picture.toImage(
      widget.viewportSize.width.toInt(),
      widget.viewportSize.height.toInt(),
    );
  }

  void _drawBackground(Canvas canvas) {
    final paint = Paint()
      ..color = widget.isDarkMode ? Colors.black : Colors.white
      ..style = PaintingStyle.fill;
    
    canvas.drawRect(
      Rect.fromLTWH(0, 0, widget.viewportSize.width, widget.viewportSize.height),
      paint,
    );
  }

  void _drawCommand(Canvas canvas, DrawCommand command) {
    switch (command.type) {
      case DrawCommandType.rect:
        _drawRect(canvas, command);
        break;
      case DrawCommandType.text:
        _drawText(canvas, command);
        break;
      case DrawCommandType.image:
        _drawImage(canvas, command);
        break;
    }
  }

  void _drawRect(Canvas canvas, DrawCommand command) {
    final paint = Paint()
      ..color = Color(command.color)
      ..style = PaintingStyle.fill;
    
    canvas.drawRect(
      Rect.fromLTWH(command.x, command.y, command.w, command.h),
      paint,
    );
  }

  void _drawText(Canvas canvas, DrawCommand command) {
    final textStyle = TextStyle(
      color: Color(command.color),
      fontSize: command.size,
      fontFamily: command.font.isNotEmpty ? command.font : null,
    );
    
    final textSpan = TextSpan(
      text: command.content,
      style: textStyle,
    );
    
    final textPainter = TextPainter(
      text: textSpan,
      textDirection: TextDirection.ltr,
    );
    
    textPainter.layout();
    textPainter.paint(canvas, Offset(command.x, command.y));
  }

  void _drawImage(Canvas canvas, DrawCommand command) {
    // TODO: Implement image loading and drawing
    // For now, draw a placeholder rectangle
    final paint = Paint()
      ..color = Colors.grey
      ..style = PaintingStyle.fill;
    
    canvas.drawRect(
      Rect.fromLTWH(command.x, command.y, 100, 100),
      paint,
    );
  }

  @override
  Widget build(BuildContext context) {
    if (_isRendering) {
      return const Center(
        child: CircularProgressIndicator(),
      );
    }

    if (_errorMessage != null) {
      return Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.error, size: 48, color: Colors.red),
            const SizedBox(height: 16),
            Text(
              _errorMessage!,
              style: const TextStyle(color: Colors.red),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 16),
            ElevatedButton(
              onPressed: _renderContent,
              child: const Text('Retry'),
            ),
          ],
        ),
      );
    }

    if (_renderedImage == null) {
      return const Center(
        child: Text('No content to display'),
      );
    }

    return CustomPaint(
      painter: _ImagePainter(_renderedImage!),
      size: widget.viewportSize,
    );
  }
}

class _ImagePainter extends CustomPainter {
  final ui.Image image;

  _ImagePainter(this.image);

  @override
  void paint(Canvas canvas, Size size) {
    canvas.drawImage(image, Offset.zero, Paint());
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) => false;
}

 