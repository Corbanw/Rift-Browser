import 'package:flutter/material.dart';
import 'engine_bridge.dart';
import 'models/layout_box.dart';
import 'web_renderer.dart';
import 'package:http/http.dart' as http;
import 'dart:ffi' as ffi;
import 'dart:async';
import 'dart:io';
import 'dev_console.dart';

void logPrint(Object? obj) {
  LogManager().add(obj?.toString() ?? 'null');
  // Optionally, also print to console
  // ignore: avoid_print
  print(obj);
}

void main() {
  logPrint('[DART] main() started');
  _setupErrorHandling();
  runApp(const BrowserApp());
  logPrint('[DART] main() finished');
}

void _setupErrorHandling() {
  FlutterError.onError = (FlutterErrorDetails details) {
    FlutterError.dumpErrorToConsole(details);
    logPrint('[DART] FlutterError: \\${details.exceptionAsString()}');
    if (details.stack != null) logPrint(details.stack);
  };
}

class BrowserApp extends StatelessWidget {
  const BrowserApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Rift Browser',
      theme: _createAppTheme(),
      home: const BrowserScreen(),
    );
  }

  ThemeData _createAppTheme() {
    return ThemeData(
      primarySwatch: Colors.blue,
      useMaterial3: true,
      brightness: Brightness.light,
      scaffoldBackgroundColor: Colors.white,
      appBarTheme: const AppBarTheme(
        backgroundColor: Colors.blue,
        foregroundColor: Colors.white,
        elevation: 2,
      ),
    );
  }
}

class BrowserScreen extends StatefulWidget {
  const BrowserScreen({super.key});

  @override
  State<BrowserScreen> createState() => _BrowserScreenState();
}

class _BrowserScreenState extends State<BrowserScreen> {
  // State variables
  bool _engineReady = false;
  List<LayoutBox> _layoutBoxes = [];
  bool _isLoading = false;
  double _loadingProgress = 0.0;
  String _loadingMessage = '';
  late TextEditingController _urlController;
  late ScrollController _scrollController;
  String _statusMessage = 'Ready';
  bool _showDevConsole = false;
  bool _isDarkMode = false; // Dark mode state
  String _currentUrl = '';
  String _errorMessage = '';
  bool _hasError = false;
  
  // URL suggestions and auto-complete
  List<String> _urlSuggestions = [];
  bool _showSuggestions = false;
  final List<String> _browsingHistory = [];
  
  // Common domains for suggestions
  static const List<String> _commonDomains = [
    'https://google.com',
    'https://youtube.com',
    'https://github.com',
    'https://stackoverflow.com',
    'https://reddit.com',
    'https://wikipedia.org',
    'https://amazon.com',
    'https://facebook.com',
    'https://twitter.com',
    'https://linkedin.com',
    'https://example.com',
    'https://httpbin.org',
  ];
  
  // Constants
  static const int _maxHtmlSize = 2000000;
  static const int _maxLayoutBoxes = 50;
  static const int _pageLoadTimeoutSeconds = 15;
  static const int _loadingMessageClearDelaySeconds = 2;

  @override
  void initState() {
    super.initState();
    _initializeControllers();
    _setupScrollListener();
    _initializeEngine();
    _setupLogRedirection();
  }

  void _initializeControllers() {
    _urlController = TextEditingController();
    _scrollController = ScrollController();
  }

  void _setupScrollListener() {
    _scrollController.addListener(() {
      if (mounted) {
        setState(() {
          // Trigger rebuild to update scroll offset
        });
      }
    });
  }

  void _setupLogRedirection() {
    // Redirect Flutter errors to LogManager
    FlutterError.onError = (FlutterErrorDetails details) {
      LogManager().add('FlutterError: \\${details.exceptionAsString()}\\n\\${details.stack ?? ''}');
      FlutterError.dumpErrorToConsole(details);
    };
  }

  Future<void> _initializeEngine() async {
    _updateInitializationStatus('Initializing Rust engine...', false);
    
    try {
      final result = EngineBridge.initialize();
      _handleInitializationResult(result);
    } catch (e, stack) {
      _handleInitializationError(e, stack);
    }
  }

  void _updateInitializationStatus(String message, bool ready) {
    setState(() {
      _statusMessage = message;
      _engineReady = ready;
    });
  }

  void _handleInitializationResult(EngineInitResult result) {
    if (!result.success) {
      final errorMessage = 'Failed to initialize Rust engine:\n\\${result.errorMessage ?? ''}\n\\${result.stackTrace ?? ''}';
      _updateInitializationStatus(errorMessage, false);
      logPrint('[ERROR] main.dart:_initializeEngine: \\${result.errorMessage}\n\\${result.stackTrace}');
      return;
    }
    _updateInitializationStatus('Rust engine initialized successfully', true);
  }

  void _handleInitializationError(dynamic error, StackTrace stack) {
    final errorMessage = 'Error initializing Rust engine: \\$error\n\\$stack';
    _updateInitializationStatus(errorMessage, false);
    logPrint('[ERROR] main.dart:_initializeEngine: \\$error\n\\$stack');
  }

  Future<void> _loadPage(String url) async {
    _startLoading();
    
    // Clear previous page data to prevent memory accumulation
    _clearPreviousPageData();
    
    // Add to browsing history
    _addToHistory(url);
    
    try {
      logPrint('[DART] _loadPage called with url: $url');
      
      final overallTimeout = _createPageLoadTimeout();
      final loadOperation = _performLoadOperation(url);
      
      // Race between the operation and timeout
      await Future.any([loadOperation, overallTimeout]);
      
    } catch (e) {
      _handleLoadError(e, url);
    }
  }

  void _addToHistory(String url) {
    // Normalize URL
    String normalizedUrl = url;
    if (!url.startsWith('http')) {
      normalizedUrl = 'https://$url';
    }
    
    // Add to history if not already the last entry
    if (_browsingHistory.isEmpty || _browsingHistory.last != normalizedUrl) {
      _browsingHistory.add(normalizedUrl);
      // Keep only last 50 entries
      if (_browsingHistory.length > 50) {
        _browsingHistory.removeAt(0);
      }
    }
    
    _currentUrl = normalizedUrl;
  }

  void _handleLoadError(dynamic error, String url) {
    logPrint('[DART] _loadPage: Error: $error');
    
    String errorMessage = error.toString();
    
    // Categorize errors for better error pages
    if (error is TimeoutException) {
      errorMessage = 'Request timeout - the page took too long to load';
    } else if (errorMessage.contains('SocketException')) {
      errorMessage = 'Network error - unable to connect to the server';
    } else if (errorMessage.contains('404')) {
      errorMessage = 'Page not found (404) - the requested page does not exist';
    } else if (errorMessage.contains('403')) {
      errorMessage = 'Access forbidden (403) - you do not have permission to access this page';
    } else if (errorMessage.contains('500')) {
      errorMessage = 'Server error (500) - the server encountered an internal error';
    } else if (errorMessage.contains('SSL') || errorMessage.contains('certificate')) {
      errorMessage = 'Security error - SSL certificate verification failed';
    }
    
    setState(() {
      _isLoading = false;
      _hasError = true;
      _errorMessage = errorMessage;
      _currentUrl = url;
    });
  }

  void _startLoading() {
    setState(() {
      _isLoading = true;
      _loadingProgress = 0.0;
      _loadingMessage = 'Fetching page...';
    });
  }

  Future<void> _createPageLoadTimeout() {
    return Future.delayed(const Duration(seconds: _pageLoadTimeoutSeconds), () {
      throw TimeoutException('Page loading timed out after $_pageLoadTimeoutSeconds seconds');
    });
  }

  Future<void> _performLoadOperation(String url) async {
    try {
      _updateLoadingProgress(0.2, 'Fetching and parsing URL via Rust...');
      
      // Use Rust to fetch HTML, CSS, and create layout boxes directly
      final layoutBoxes = await EngineBridge.parseUrlViaRust(url);
      
      // Extract layout boxes from Rust engine
      final limitedBoxes = _limitLayoutBoxes(layoutBoxes);
      
      // Finalize loading
      _finalizeLoading(limitedBoxes);
      
    } catch (e) {
      logPrint('[DART] _performLoadOperation: Error: $e');
      _handleLoadError(e, url);
    }
  }

  bool _isHtmlTooComplex(String html) {
    if (html.length > _maxHtmlSize) {
      logPrint('[DART] HTML too complex (${html.length} chars), showing simplified view');
      return true;
    }
    return false;
  }

  void _showSimplifiedView(String html) {
    setState(() {
      _layoutBoxes = _createSimplifiedLayoutBoxes(html);
      _isLoading = false;
      _loadingProgress = 1.0;
      _loadingMessage = 'Simplified view loaded';
    });
  }

  List<LayoutBox> _createSimplifiedLayoutBoxes(String html) {
    return [
      LayoutBox(
        x: 10.0, y: 10.0, width: 780.0, height: 100.0,
        nodeType: 'div', 
        textContent: 'Page too complex to render completely.\nThis page has ${html.length} characters of HTML.\nShowing simplified view.',
        backgroundColor: '#f0f0f0', color: '#333', fontSize: 14.0,
        fontFamily: 'Arial', borderWidth: 1.0, borderColor: '#ccc',
        padding: 10.0, margin: 10.0, fontWeight: 400.0, textAlign: 'left',
      ),
      LayoutBox(
        x: 10.0, y: 120.0, width: 780.0, height: 50.0,
        nodeType: 'div', textContent: 'Try visiting a simpler webpage.',
        backgroundColor: '#e8f4fd', color: '#0066cc', fontSize: 12.0,
        fontFamily: 'Arial', borderWidth: 0.0, borderColor: '',
        padding: 10.0, margin: 10.0, fontWeight: 400.0, textAlign: 'left',
      ),
    ];
  }

  Future<List<LayoutBox>> _parseHtmlWithProgress(String html) async {
    try {
      _updateLoadingProgress(0.4, 'Parsing HTML...');
      final layoutBoxes = await EngineBridge.parseHtml(html);
      return layoutBoxes;
    } catch (e) {
      logPrint('[DART] _parseHtmlWithProgress: Error: $e');
      rethrow;
    }
  }

  Future<List<LayoutBox>> _extractLayoutBoxesWithProgress(ffi.Pointer<ffi.Void> resultPtr) async {
    try {
      _updateLoadingProgress(0.6, 'Extracting layout boxes...');
      
      // Add timeout protection for layout box extraction
      final layoutBoxes = await Future.any([
        _safeExtractLayoutBoxes(resultPtr),
        Future.delayed(const Duration(seconds: 5), () {
          logPrint('[DART] Layout box extraction timed out after 5 seconds');
          return <LayoutBox>[];
        }),
      ]);
      
      // If no layout boxes were extracted, show fallback
      if (layoutBoxes.isEmpty) {
        logPrint('[DART] No layout boxes extracted, showing fallback');
        return _createFallbackLayoutBoxes('No content could be extracted from this page');
      }
      
      return layoutBoxes;
    } catch (e) {
      logPrint('[DART] _extractLayoutBoxesWithProgress: Error: $e');
      return _createFallbackLayoutBoxes('Error extracting page content: $e');
    }
  }

  Future<List<LayoutBox>> _safeExtractLayoutBoxes(ffi.Pointer<ffi.Void> resultPtr) async {
    try {
      return await EngineBridge.extractLayoutBoxesAsync(
        resultPtr,
        _updateLayoutBoxProgress,
      );
    } catch (e) {
      logPrint('[DART] _safeExtractLayoutBoxes: Error: $e');
      return [];
    }
  }

  void _updateLayoutBoxProgress(double progress) {
    if (mounted) {
      setState(() {
        _loadingProgress = 0.6 + (progress * 0.3); // 60% to 90%
        _loadingMessage = 'Processing layout boxes... ${(progress * 100).toInt()}%';
      });
    }
  }

  List<LayoutBox> _limitLayoutBoxes(List<LayoutBox> layoutBoxes) {
    final originalCount = layoutBoxes.length;
    
    // Limit to prevent UI freezing
    if (originalCount > _maxLayoutBoxes) {
      final limitedBoxes = layoutBoxes.take(_maxLayoutBoxes).toList();
      
      // Log warning
      final warningMessage = '[WARNING] Layout box limit reached: $originalCount boxes found, limited to $_maxLayoutBoxes. Some content may not be displayed.';
      logPrint(warningMessage);
      LogManager().add(warningMessage);
      
      // Add a warning box to the layout
      limitedBoxes.add(_createWarningLayoutBox(originalCount, _maxLayoutBoxes));
      
      return limitedBoxes;
    }
    
    // Check for Rust engine limit warnings
    _checkForRustEngineWarnings(layoutBoxes);
    
    return layoutBoxes;
  }

  LayoutBox _createWarningLayoutBox(int originalCount, int maxCount) {
    return LayoutBox(
      x: 10.0, y: 10.0, width: 780.0, height: 40.0,
      nodeType: 'div', 
      textContent: 'WARNING: Page too complex. Showing $maxCount of $originalCount elements.',
      backgroundColor: '#fff3cd', color: '#856404', fontSize: 12.0,
      fontFamily: 'Arial', borderWidth: 1.0, borderColor: '#ffeaa7',
      padding: 8.0, margin: 5.0, fontWeight: 600.0, textAlign: 'center',
    );
  }

  void _checkForRustEngineWarnings(List<LayoutBox> layoutBoxes) {
    if (layoutBoxes.length < 5) {
      final warningMessage = '[WARNING] Very few layout boxes (${layoutBoxes.length}) were generated. The Rust engine may have hit its internal limits (500 boxes, 1000 DOM nodes). Some page content may not be displayed.';
      logPrint(warningMessage);
      LogManager().add(warningMessage);
      
      // Add a warning box if we don't already have one
      if (!layoutBoxes.any((box) => box.nodeType == 'warning')) {
        layoutBoxes.add(_createRustEngineWarningLayoutBox());
      }
    }
  }

  LayoutBox _createRustEngineWarningLayoutBox() {
    return LayoutBox(
      x: 10.0, y: 10.0, width: 780.0, height: 40.0,
      nodeType: 'warning', 
      textContent: 'WARNING: Rust engine limits reached. Page may not render completely.',
      backgroundColor: '#f8d7da', color: '#721c24', fontSize: 12.0,
      fontFamily: 'Arial', borderWidth: 1.0, borderColor: '#f5c6cb',
      padding: 8.0, margin: 5.0, fontWeight: 600.0, textAlign: 'center',
    );
  }

  void _finalizeLoading(List<LayoutBox> layoutBoxes) {
    setState(() {
      _layoutBoxes = layoutBoxes;
      _isLoading = false;
      _loadingProgress = 1.0;
      _loadingMessage = 'Page loaded successfully!';
      _hasError = false;
      _errorMessage = '';
    });
    
    logPrint('[DART] Page loaded with ${layoutBoxes.length} layout boxes');
  }

  void _updateLoadingProgress(double progress, String message) {
    setState(() {
      _loadingProgress = progress;
      _loadingMessage = message;
    });
  }

  List<LayoutBox> _createFallbackLayoutBoxes(String message) {
    return [
      LayoutBox(
        x: 10.0, y: 10.0, width: 780.0, height: 60.0,
        nodeType: 'div', textContent: message,
        backgroundColor: '#f8d7da', color: '#721c24', fontSize: 14.0,
        fontFamily: 'Arial', borderWidth: 1.0, borderColor: '#f5c6cb',
        padding: 15.0, margin: 10.0, fontWeight: 400.0, textAlign: 'left',
      ),
    ];
  }

  Future<String> _fetchHtmlWithProgress(String url) async {
    try {
      _updateLoadingProgress(0.2, 'Fetching HTML...');
      final html = await _fetchHtml(url);
      return html;
    } catch (e) {
      logPrint('[DART] _fetchHtmlWithProgress: Error: $e');
      rethrow;
    }
  }

  Future<String> _fetchHtml(String url) async {
    logPrint('[DART] _fetchHtml called with url: $url');
    
    final fullUrl = _ensureUrlHasProtocol(url);
    logPrint('[DART] _fetchHtml: Fetching from $fullUrl');
    
    try {
      final response = await http.get(Uri.parse(fullUrl));
      return _handleHttpResponse(response);
    } catch (e) {
      _handleNetworkError(e);
      rethrow;
    }
  }

  String _ensureUrlHasProtocol(String url) {
    if (!url.startsWith('http://') && !url.startsWith('https://')) {
      return 'https://$url';
    }
    return url;
  }

  String _handleHttpResponse(http.Response response) {
    if (response.statusCode == 200) {
      logPrint('[DART] _fetchHtml: Successfully fetched ${response.body.length} characters');
      return response.body;
    } else {
      logPrint('[ERROR] _fetchHtml: HTTP ${response.statusCode}');
      throw Exception('HTTP ${response.statusCode}: ${response.reasonPhrase}');
    }
  }

  void _handleNetworkError(dynamic error) {
    logPrint('[ERROR] _fetchHtml: Network error: $error');
    throw Exception('Failed to fetch HTML: $error');
  }

  @override
  Widget build(BuildContext context) {
    logPrint('[DART] build() called. Engine ready: $_engineReady, Layout boxes: ${_layoutBoxes.length}');
    
    if (!_engineReady && _statusMessage.startsWith('Initializing')) {
      return _buildInitializingScreen();
    }
    
    if (!_engineReady) {
      return _buildErrorScreen();
    }
    
    return _buildMainScreen();
  }

  Widget _buildInitializingScreen() {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const CircularProgressIndicator(),
            const SizedBox(height: 24),
            Text(_statusMessage, style: const TextStyle(fontSize: 18)),
          ],
        ),
      ),
    );
  }

  Widget _buildErrorScreen() {
    return Scaffold(
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Icon(Icons.error, color: Colors.red, size: 48),
            const SizedBox(height: 16),
            const Text('Failed to initialize Rust engine', style: TextStyle(fontSize: 20, color: Colors.red)),
            const SizedBox(height: 8),
            Text(_statusMessage, style: const TextStyle(fontSize: 14)),
          ],
        ),
      ),
    );
  }

  Widget _buildMainScreen() {
    return Scaffold(
      appBar: _buildAppBar(),
      body: _showDevConsole 
        ? Row(
            children: [
              // Left half - Website content
              Expanded(
                child: Column(
                  children: [
                    _buildUrlBar(),
                    if (_isLoading) _buildLoadingIndicator(),
                    Expanded(child: _buildContentArea()),
                  ],
                ),
              ),
              // Right half - Dev Console
              DevConsole(onClose: _toggleDevConsole),
            ],
          )
        : Column(
            children: [
              _buildUrlBar(),
              if (_isLoading) _buildLoadingIndicator(),
              Expanded(child: _buildContentArea()),
            ],
          ),
      floatingActionButton: FloatingActionButton(
        onPressed: _toggleDevConsole,
        tooltip: 'Open Dev Console',
        child: const Icon(Icons.developer_mode),
      ),
    );
  }

  PreferredSizeWidget _buildAppBar() {
    return AppBar(
      title: const Text('Rift Browser'),
      backgroundColor: _isDarkMode ? Colors.grey[800] : Colors.blue,
      foregroundColor: Colors.white,
      actions: [
        IconButton(
          onPressed: _toggleDarkMode,
          icon: Icon(_isDarkMode ? Icons.light_mode : Icons.dark_mode),
          tooltip: _isDarkMode ? 'Switch to Light Mode' : 'Switch to Dark Mode',
        ),
      ],
    );
  }

  void _toggleDarkMode() {
    setState(() {
      _isDarkMode = !_isDarkMode;
    });
  }

  Widget _buildUrlBar() {
    return Container(
      padding: const EdgeInsets.all(12.0),
      decoration: BoxDecoration(
        color: _isDarkMode ? Colors.grey[800] : Colors.grey[100],
        border: Border(
          bottom: BorderSide(
            color: _isDarkMode ? Colors.grey[600]! : Colors.grey[300]!,
            width: 1,
          ),
        ),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.1),
            blurRadius: 4,
            offset: const Offset(0, 2),
          ),
        ],
      ),
      child: Column(
        children: [
          Row(
            children: [
              // Back button
              IconButton(
                onPressed: _canGoBack() ? _goBack : null,
                icon: Icon(
                  Icons.arrow_back,
                  color: _canGoBack() ? (_isDarkMode ? Colors.white : Colors.blue) : Colors.grey,
                ),
                tooltip: 'Go back',
              ),
              // Forward button
              IconButton(
                onPressed: _canGoForward() ? _goForward : null,
                icon: Icon(
                  Icons.arrow_forward,
                  color: _canGoForward() ? (_isDarkMode ? Colors.white : Colors.blue) : Colors.grey,
                ),
                tooltip: 'Go forward',
              ),
              // Refresh button
              IconButton(
                onPressed: _currentUrl.isNotEmpty ? _refreshPage : null,
                icon: Icon(
                  Icons.refresh,
                  color: _currentUrl.isNotEmpty ? (_isDarkMode ? Colors.white : Colors.blue) : Colors.grey,
                ),
                tooltip: 'Refresh page',
              ),
              const SizedBox(width: 8),
              // URL field
              Expanded(child: _buildUrlTextField()),
              const SizedBox(width: 8),
              // Go button
              _buildGoButton(),
            ],
          ),
          // URL suggestions
          if (_showSuggestions && _urlSuggestions.isNotEmpty)
            _buildUrlSuggestions(),
        ],
      ),
    );
  }

  Widget _buildUrlTextField() {
    return Container(
      decoration: BoxDecoration(
        color: _isDarkMode ? Colors.grey[700] : Colors.white,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(
          color: _isDarkMode ? Colors.grey[600]! : Colors.grey[300]!,
        ),
      ),
      child: TextField(
        controller: _urlController,
        style: TextStyle(
          color: _isDarkMode ? Colors.white : Colors.black,
          fontSize: 14,
        ),
        decoration: InputDecoration(
          hintText: 'Enter URL or search...',
          hintStyle: TextStyle(
            color: _isDarkMode ? Colors.grey[400] : Colors.grey[500],
          ),
          prefixIcon: Icon(
            Icons.search,
            color: _isDarkMode ? Colors.grey[400] : Colors.grey[500],
            size: 20,
          ),
          border: InputBorder.none,
          contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
        ),
        onChanged: _onUrlChanged,
        onSubmitted: _handleUrlSubmission,
        onTap: _onUrlFieldTapped,
      ),
    );
  }

  Widget _buildUrlSuggestions() {
    return Container(
      margin: const EdgeInsets.only(top: 4),
      decoration: BoxDecoration(
        color: _isDarkMode ? Colors.grey[800] : Colors.white,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(
          color: _isDarkMode ? Colors.grey[600]! : Colors.grey[300]!,
        ),
        boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.1),
            blurRadius: 8,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxHeight: 200),
        child: ListView.builder(
          shrinkWrap: true,
          itemCount: _urlSuggestions.length,
          itemBuilder: (context, index) {
            final suggestion = _urlSuggestions[index];
            return ListTile(
              dense: true,
              leading: Icon(
                _getSuggestionIcon(suggestion),
                size: 16,
                color: _isDarkMode ? Colors.grey[400] : Colors.grey[600],
              ),
              title: Text(
                suggestion,
                style: TextStyle(
                  fontSize: 13,
                  color: _isDarkMode ? Colors.white : Colors.black,
                ),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
              onTap: () => _selectSuggestion(suggestion),
            );
          },
        ),
      ),
    );
  }

  IconData _getSuggestionIcon(String url) {
    if (url.contains('google.com')) return Icons.search;
    if (url.contains('youtube.com')) return Icons.play_circle_outline;
    if (url.contains('github.com')) return Icons.code;
    if (url.contains('stackoverflow.com')) return Icons.help_outline;
    if (url.contains('reddit.com')) return Icons.forum;
    if (url.contains('wikipedia.org')) return Icons.book;
    if (url.contains('amazon.com')) return Icons.shopping_cart;
    if (url.contains('facebook.com')) return Icons.facebook;
    if (url.contains('twitter.com')) return Icons.flutter_dash;
    if (url.contains('linkedin.com')) return Icons.work;
    return Icons.language;
  }

  void _onUrlChanged(String value) {
    _generateSuggestions(value);
  }

  void _onUrlFieldTapped() {
    if (_urlController.text.isNotEmpty) {
      _generateSuggestions(_urlController.text);
    }
  }

  void _generateSuggestions(String input) {
    if (input.isEmpty) {
      setState(() {
        _urlSuggestions = _commonDomains.take(6).toList();
        _showSuggestions = true;
      });
      return;
    }

    final suggestions = <String>[];
    
    // Add history matches
    for (final history in _browsingHistory) {
      if (history.toLowerCase().contains(input.toLowerCase())) {
        suggestions.add(history);
      }
    }
    
    // Add common domain matches
    for (final domain in _commonDomains) {
      if (domain.toLowerCase().contains(input.toLowerCase())) {
        suggestions.add(domain);
      }
    }
    
    // Add search suggestions if it looks like a search query
    if (!input.startsWith('http') && !input.contains('.') && input.length > 2) {
      suggestions.add('https://google.com/search?q=${Uri.encodeComponent(input)}');
    }
    
    // Add protocol if missing
    if (!input.startsWith('http') && input.contains('.') && !input.startsWith('www.')) {
      suggestions.add('https://$input');
    }
    
    setState(() {
      _urlSuggestions = suggestions.take(8).toList();
      _showSuggestions = suggestions.isNotEmpty;
    });
  }

  void _selectSuggestion(String suggestion) {
    _urlController.text = suggestion;
    setState(() {
      _showSuggestions = false;
    });
    _loadPage(suggestion);
  }

  bool _canGoBack() {
    return _browsingHistory.length > 1;
  }

  bool _canGoForward() {
    return false; // TODO: Implement forward history
  }

  void _goBack() {
    if (_browsingHistory.length > 1) {
      _browsingHistory.removeLast();
      final previousUrl = _browsingHistory.last;
      _urlController.text = previousUrl;
      _loadPage(previousUrl);
    }
  }

  void _goForward() {
    // TODO: Implement forward navigation
  }

  void _refreshPage() {
    if (_currentUrl.isNotEmpty) {
      _loadPage(_currentUrl);
    }
  }

  void _handleUrlSubmission(String url) {
    if (url.isNotEmpty) {
      setState(() {
        _showSuggestions = false;
      });
      _loadPage(url);
    }
  }

  Widget _buildGoButton() {
    return ElevatedButton(
      onPressed: _handleGoButtonPressed,
      child: const Text('Go'),
    );
  }

  void _handleGoButtonPressed() {
    final url = _urlController.text;
    if (url.isNotEmpty) {
      _loadPage(url);
    }
  }

  Widget _buildLoadingIndicator() {
    return Container(
      padding: const EdgeInsets.all(16.0),
      child: Column(
        children: [
          LinearProgressIndicator(
            value: _loadingProgress,
            backgroundColor: Colors.grey[300],
            valueColor: const AlwaysStoppedAnimation<Color>(Colors.blue),
          ),
          const SizedBox(height: 8),
          Text(
            _loadingMessage,
            style: TextStyle(fontSize: 14, color: Colors.grey[600]),
          ),
        ],
      ),
    );
  }

  Widget _buildContentArea() {
    if (_hasError) {
      return _buildErrorPage();
    }
    
    if (_isLoading) {
      return _buildLoadingContent();
    }
    
    if (_layoutBoxes.isEmpty) {
      return _buildEmptyContent();
    }
    
    return _buildPageContent();
  }

  Widget _buildErrorPage() {
    return Center(
      child: Container(
        padding: const EdgeInsets.all(32.0),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              _getErrorIcon(),
              size: 80,
              color: _getErrorColor(),
            ),
            const SizedBox(height: 24),
            Text(
              _getErrorTitle(),
              style: TextStyle(
                fontSize: 24,
                fontWeight: FontWeight.bold,
                color: _getErrorColor(),
              ),
            ),
            const SizedBox(height: 16),
            Text(
              _errorMessage,
              style: TextStyle(
                fontSize: 16,
                color: _isDarkMode ? Colors.grey[300] : Colors.grey[600],
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 32),
            Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                ElevatedButton.icon(
                  onPressed: _refreshPage,
                  icon: const Icon(Icons.refresh),
                  label: const Text('Try Again'),
                  style: ElevatedButton.styleFrom(
                    backgroundColor: _getErrorColor(),
                    foregroundColor: Colors.white,
                    padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
                  ),
                ),
                const SizedBox(width: 16),
                OutlinedButton.icon(
                  onPressed: () {
                    setState(() {
                      _hasError = false;
                      _errorMessage = '';
                    });
                  },
                  icon: const Icon(Icons.home),
                  label: const Text('Go Home'),
                  style: OutlinedButton.styleFrom(
                    padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 12),
                  ),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  IconData _getErrorIcon() {
    if (_errorMessage.contains('404')) return Icons.error_outline;
    if (_errorMessage.contains('timeout')) return Icons.schedule;
    if (_errorMessage.contains('network')) return Icons.wifi_off;
    if (_errorMessage.contains('SSL') || _errorMessage.contains('certificate')) return Icons.security;
    return Icons.error;
  }

  Color _getErrorColor() {
    if (_errorMessage.contains('404')) return Colors.orange;
    if (_errorMessage.contains('timeout')) return Colors.amber;
    if (_errorMessage.contains('network')) return Colors.red;
    if (_errorMessage.contains('SSL') || _errorMessage.contains('certificate')) return Colors.purple;
    return Colors.red;
  }

  String _getErrorTitle() {
    if (_errorMessage.contains('404')) return 'Page Not Found';
    if (_errorMessage.contains('timeout')) return 'Request Timeout';
    if (_errorMessage.contains('network')) return 'Network Error';
    if (_errorMessage.contains('SSL') || _errorMessage.contains('certificate')) return 'Security Error';
    return 'Something Went Wrong';
  }

  Widget _buildLoadingContent() {
    return const Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          CircularProgressIndicator(),
          SizedBox(height: 16),
          Text('Loading page...'),
        ],
      ),
    );
  }

  Widget _buildEmptyContent() {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Icon(Icons.language, size: 64, color: Colors.grey),
          const SizedBox(height: 16),
          Text(
            'Enter a URL above to load a webpage',
            style: TextStyle(fontSize: 16, color: Colors.grey[600]),
          ),
        ],
      ),
    );
  }

  Widget _buildPageContent() {
    return Column(
      children: [
        if (_layoutBoxes.isNotEmpty) _buildPerformanceInfoBar(),
        Expanded(child: _buildPageRenderer()),
      ],
    );
  }

  Widget _buildPerformanceInfoBar() {
    final hasWarnings = _layoutBoxes.any((box) => 
        box.textContent.contains('Layout limit reached') || 
        box.textContent.contains('Rust engine limit reached'));
    
    return Container(
      padding: const EdgeInsets.all(8.0),
      color: hasWarnings ? Colors.orange[50] : Colors.grey[100],
      child: Row(
        children: [
          Icon(
            hasWarnings ? Icons.warning : Icons.info_outline, 
            size: 16, 
            color: hasWarnings ? Colors.orange[700] : Colors.grey[600]
          ),
          const SizedBox(width: 8),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(
                  'Dynamic Web Renderer: ${_layoutBoxes.length} elements',
                  style: TextStyle(
                    fontSize: 12, 
                    color: hasWarnings ? Colors.orange[700] : Colors.grey[600],
                    fontWeight: hasWarnings ? FontWeight.bold : FontWeight.normal,
                  ),
                ),
                if (hasWarnings)
                  Text(
                    '⚠️ Layout limits reached - some content may be missing',
                    style: TextStyle(
                      fontSize: 10, 
                      color: Colors.orange[700],
                      fontStyle: FontStyle.italic,
                    ),
                  ),
              ],
            ),
          ),
          Text(
            'Scroll: ${_getScrollOffset()}',
            style: TextStyle(fontSize: 12, color: Colors.grey[600]),
          ),
        ],
      ),
    );
  }

  String _getScrollOffset() {
    return _scrollController.hasClients ? _scrollController.offset.toStringAsFixed(0) : "0";
  }

  Widget _buildPageRenderer() {
    return LayoutBuilder(
      builder: (context, constraints) {
        try {
          return SingleChildScrollView(
            controller: _scrollController,
            child: _buildSafeRenderer(constraints),
          );
        } catch (e) {
          logPrint('[DART] _buildPageRenderer: Error: $e');
          return _buildRenderErrorWidget(e.toString());
        }
      },
    );
  }

  Widget _buildSafeRenderer(BoxConstraints constraints) {
    try {
      return WebRenderer(
        layoutBoxes: _layoutBoxes,
        scrollOffset: _getScrollOffsetValue(),
        viewportSize: Size(constraints.maxWidth, constraints.maxHeight),
        isDarkMode: _isDarkMode,
        maxElementsToRender: 500,
      );
    } catch (e) {
      logPrint('[DART] WebRenderer failed, using fallback: $e');
      return _buildFallbackRenderer(constraints);
    }
  }

  Widget _buildFallbackRenderer(BoxConstraints constraints) {
    try {
      // Simple fallback that just shows text content
      return Container(
        width: constraints.maxWidth,
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Web Renderer Fallback (${_layoutBoxes.length} elements)',
              style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 16),
            ..._layoutBoxes.take(20).map((box) => Padding(
              padding: const EdgeInsets.symmetric(vertical: 4.0),
              child: Text(
                '${box.nodeType}: ${box.textContent.isNotEmpty ? box.textContent : '[No text]'}',
                style: const TextStyle(fontSize: 12),
              ),
            )),
            if (_layoutBoxes.length > 20)
              Text(
                '... and ${_layoutBoxes.length - 20} more elements',
                style: const TextStyle(fontSize: 12, fontStyle: FontStyle.italic),
              ),
          ],
        ),
      );
    } catch (e) {
      logPrint('[DART] Fallback renderer also failed: $e');
      return const Center(
        child: Text('Failed to render page content'),
      );
    }
  }

  Widget _buildRenderErrorWidget(String error) {
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
          const SizedBox(height: 16),
          ElevatedButton(
            onPressed: () {
              setState(() {
                _layoutBoxes = [];
              });
            },
            child: const Text('Clear Page'),
          ),
        ],
      ),
    );
  }

  double _getScrollOffsetValue() {
    return _scrollController.hasClients ? _scrollController.offset : 0.0;
  }

  void _toggleDevConsole() {
    setState(() {
      _showDevConsole = !_showDevConsole;
    });
  }

  void _log(String message) {
    LogManager().add(message);
  }

  void _clearPreviousPageData() {
    // Clear previous layout boxes to free memory
    setState(() {
      _layoutBoxes = [];
    });
    
    // Log memory usage for monitoring
    _logMemoryUsage('After clearing previous page data');
    
    // Force garbage collection if possible
    // Note: This is a hint to the garbage collector
    if (mounted) {
      // Trigger a rebuild to help with memory cleanup
      setState(() {});
    }
  }

  void _logMemoryUsage(String context) {
    try {
      // Get memory info if available
      final memoryInfo = ProcessInfo.currentRss;
      logPrint('[MEMORY] $context - RSS: ${(memoryInfo / 1024 / 1024).toStringAsFixed(2)} MB');
    } catch (e) {
      logPrint('[MEMORY] $context - Unable to get memory info: $e');
    }
  }

  @override
  void dispose() {
    _urlController.dispose();
    _scrollController.dispose();
    super.dispose();
  }
}

class HtmlElement {
  final String tag;
  final String textContent;
  
  HtmlElement({required this.tag, required this.textContent});
}

