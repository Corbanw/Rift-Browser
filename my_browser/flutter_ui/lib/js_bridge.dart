// Conditional import for web vs other platforms
export 'js_bridge_stub.dart'
    if (dart.library.js) 'js_bridge_web.dart';