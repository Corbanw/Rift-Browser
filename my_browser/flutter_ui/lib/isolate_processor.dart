import 'dart:isolate';
import 'dart:async';
import 'engine_bridge.dart';
import 'models/layout_box.dart';

class IsolateProcessor {
  static const int _maxLayoutBoxes = 1000;
  static const Duration _extractionTimeout = Duration(seconds: 30);

  // Message types for isolate communication
  static const String _processHtmlMessage = 'process_html';
  static const String _resultMessage = 'result';
  static const String _errorMessage = 'error';
  static const String _progressMessage = 'progress';

  static Future<List<LayoutBox>> processHtml(
    String html, {
    Function(double, String)? onProgress,
  }) async {
    try {
      // For now, use the main thread processing since FFI doesn't work well in isolates
      // This is a simplified version that processes HTML directly
      
      if (onProgress != null) {
        onProgress(0.1, 'Initializing processing...');
      }
      
      // Initialize engine if needed
      final initResult = EngineBridge.initialize();
      if (!initResult.success) {
        throw Exception('Failed to initialize engine: ${initResult.errorMessage}');
      }
      
      if (onProgress != null) {
        onProgress(0.3, 'Parsing HTML...');
      }
      
      // Parse HTML using the engine bridge
      final layoutBoxes = await EngineBridge.parseHtml(html);
      
      if (onProgress != null) {
        onProgress(0.8, 'Processing layout boxes...');
      }
      
      // Limit the number of layout boxes to prevent UI freezing
      final limitedBoxes = layoutBoxes.take(_maxLayoutBoxes).toList();
      
      if (onProgress != null) {
        onProgress(1.0, 'Processing complete');
      }
      
      return limitedBoxes;
    } catch (e) {
      throw Exception('Failed to process HTML: $e');
    }
  }

  static Future<List<LayoutBox>> processHtmlInIsolate(String html, String url) async {
    // For now, use the simplified processing since FFI isolates have limitations
    return processHtml(html);
  }

  static void _isolateEntryPoint(SendPort mainSendPort) async {
    final receivePort = ReceivePort();
    mainSendPort.send(receivePort.sendPort);
    
    await for (final message in receivePort) {
      if (message is Map<String, dynamic>) {
        switch (message['type']) {
          case _processHtmlMessage:
            try {
              final result = await processHtml(message['html']);
              mainSendPort.send({
                'type': _resultMessage,
                'data': result,
              });
            } catch (e) {
              mainSendPort.send({
                'type': _errorMessage,
                'error': e.toString(),
              });
            }
            break;
        }
      }
    }
  }
}

// Helper class for progress tracking
class ProcessingProgress {
  final double progress;
  final String message;
  
  ProcessingProgress(this.progress, this.message);
}
