import 'package:flutter/material.dart';
import 'engine_bridge.dart';
import 'models/layout_box.dart';
import 'paint_renderer.dart';
import 'package:http/http.dart' as http;
import 'dart:ffi' as ffi;
import 'dart:async';
import 'dart:io';
import 'dev_console.dart';
import 'js_bridge.dart';
import 'url_bar.dart';
import 'content_area.dart';
import 'performance_info_bar.dart';
import 'page_renderer.dart';
import 'utils/network_utils.dart';
import 'utils/error_utils.dart';
import 'utils/history_manager.dart';
import 'utils/memory_utils.dart';

// TODO: Move URL bar, content area, and helpers to their own files in next steps.

class BrowserScreen extends StatefulWidget {
  const BrowserScreen({super.key});

  @override
  State<BrowserScreen> createState() => _BrowserScreenState();
}

class _BrowserScreenState extends State<BrowserScreen> {
  // State variables
  bool _engineReady = false;
  String _currentHtml = '';
  String _currentCss = '';
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
  final BrowsingHistoryManager historyManager = BrowsingHistoryManager();
  
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

  void _initializeEngine() async {
    _updateInitializationStatus('Initializing Rust engine...', false);
    try {
      final result = EngineBridge.initialize();
      _handleInitializationResult(result);
    } catch (e, stack) {
      _handleInitializationError(e, stack);
    }
  }

  void _setupLogRedirection() {
    // Redirect Flutter errors to LogManager
    FlutterError.onError = (FlutterErrorDetails details) {
      LogManager().add('FlutterError: \\${details.exceptionAsString()}\\n\\${details.stack ?? ''}');
      FlutterError.dumpErrorToConsole(details);
    };
  }

  PreferredSizeWidget _buildAppBar() {
    return AppBar(
      title: const Text('Rift Browser'),
      backgroundColor: _isDarkMode ? Colors.grey[800] : Colors.blue,
      foregroundColor: Colors.white,
      actions: [
        IconButton(
          onPressed: _testJavaScript,
          icon: const Icon(Icons.code),
          tooltip: 'Test JavaScript',
        ),
        IconButton(
          onPressed: _toggleDarkMode,
          icon: Icon(_isDarkMode ? Icons.light_mode : Icons.dark_mode),
          tooltip: _isDarkMode ? 'Switch to Light Mode' : 'Switch to Dark Mode',
        ),
      ],
    );
  }

  bool _canGoBack() {
    return historyManager.getHistory().length > 1;
  }

  bool _canGoForward() {
    // TODO: Implement forward navigation if desired
    return false;
  }

  void _goForward() {
    // TODO: Implement forward navigation if desired
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
                    UrlBar(
                      urlController: _urlController,
                      isDarkMode: _isDarkMode,
                      canGoBack: _canGoBack(),
                      canGoForward: _canGoForward(),
                      onBack: _canGoBack() ? _goBack : null,
                      onForward: _canGoForward() ? _goForward : null,
                      onRefresh: _currentUrl.isNotEmpty ? _refreshPage : null,
                      onGo: _handleGoButtonPressed,
                      onChanged: _onUrlChanged,
                      onSubmitted: _handleUrlSubmission,
                      onTap: _onUrlFieldTapped,
                      showSuggestions: _showSuggestions,
                      urlSuggestions: _urlSuggestions,
                      onSuggestionSelected: _selectSuggestion,
                    ),
                    if (_isLoading) _buildLoadingIndicator(),
                    Expanded(child: ContentArea(
                      isLoading: _isLoading,
                      hasError: _hasError,
                      errorMessage: _errorMessage,
                      isDarkMode: _isDarkMode,
                      currentHtml: _currentHtml,
                      currentCss: _currentCss,
                      buildErrorPage: _buildErrorPage,
                      buildLoadingContent: _buildLoadingContent,
                      buildEmptyContent: _buildEmptyContent,
                      buildPageContent: _buildPageContent,
                    )),
                  ],
                ),
              ),
              // Right half - Dev Console
              DevConsole(onClose: _toggleDevConsole),
            ],
          )
        : Column(
            children: [
              UrlBar(
                urlController: _urlController,
                isDarkMode: _isDarkMode,
                canGoBack: _canGoBack(),
                canGoForward: _canGoForward(),
                onBack: _canGoBack() ? _goBack : null,
                onForward: _canGoForward() ? _goForward : null,
                onRefresh: _currentUrl.isNotEmpty ? _refreshPage : null,
                onGo: _handleGoButtonPressed,
                onChanged: _onUrlChanged,
                onSubmitted: _handleUrlSubmission,
                onTap: _onUrlFieldTapped,
                showSuggestions: _showSuggestions,
                urlSuggestions: _urlSuggestions,
                onSuggestionSelected: _selectSuggestion,
              ),
              if (_isLoading) _buildLoadingIndicator(),
              Expanded(child: ContentArea(
                isLoading: _isLoading,
                hasError: _hasError,
                errorMessage: _errorMessage,
                isDarkMode: _isDarkMode,
                currentHtml: _currentHtml,
                currentCss: _currentCss,
                buildErrorPage: _buildErrorPage,
                buildLoadingContent: _buildLoadingContent,
                buildEmptyContent: _buildEmptyContent,
                buildPageContent: _buildPageContent,
              )),
            ],
          ),
      floatingActionButton: FloatingActionButton(
        onPressed: _toggleDevConsole,
        tooltip: 'Open Dev Console',
        child: const Icon(Icons.developer_mode),
      ),
    );
  }

  Widget _buildPageContent() {
    return Column(
      children: [
        if (_currentHtml.isNotEmpty)
          PerformanceInfoBar(
            currentHtml: _currentHtml,
            currentCss: _currentCss,
            scrollOffset: _getScrollOffset(),
            isDarkMode: _isDarkMode,
          ),
        Expanded(child: PageRenderer(
          currentHtml: _currentHtml,
          currentCss: _currentCss,
          scrollController: _scrollController,
          isDarkMode: _isDarkMode,
          getScrollOffsetValue: _getScrollOffsetValue,
        )),
      ],
    );
  }

  Future<void> _performLoadOperation(String url) async {
    try {
      _updateLoadingProgress(0.2, 'Fetching and parsing URL...');
      // Fetch HTML from the URL
      final html = await fetchHtml(url, logPrint);
      // Extract inline <style> CSS
      final css = await _extractCssFromHtml(html);
      // Extract external CSS URLs
      final externalCssUrls = _extractExternalCssUrls(html, url);
      // Fetch external CSS
      final externalCssList = await Future.wait(externalCssUrls.map((u) => fetchExternalCss(u, logPrint)));
      // Combine all CSS
      final allCss = (externalCssList..add(css)).join('\n');
      // Extract image URLs
      final imageUrls = _extractImageUrls(html, allCss, url);
      // Finalize loading with HTML, all CSS, and image URLs
      _finalizeLoadingWithImages(html, allCss, imageUrls);
    } catch (e) {
      logPrint('[DART] _performLoadOperation: Error: $e');
      handleLoadError(
        setState: setState,
        logPrint: logPrint,
        error: e,
        url: url,
        setErrorMessage: (msg) => _errorMessage = msg,
        setCurrentUrl: (u) => _currentUrl = u,
      );
    }
  }

  Future<void> _loadPage(String url) async {
    _startLoading();
    // Clear previous page data to prevent memory accumulation
    _clearPreviousPageData();
    // Add to browsing history
    historyManager.addToHistory(url);
    try {
      logPrint('[DART] _loadPage called with url: $url');
      final overallTimeout = _createPageLoadTimeout();
      final loadOperation = _performLoadOperation(url);
      // Race between the operation and timeout
      await Future.any([loadOperation, overallTimeout]);
    } catch (e) {
      handleLoadError(
        setState: setState,
        logPrint: logPrint,
        error: e,
        url: url,
        setErrorMessage: (msg) => _errorMessage = msg,
        setCurrentUrl: (u) => _currentUrl = u,
      );
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
    for (final history in historyManager.getHistory()) {
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

  void _goBack() {
    final history = historyManager.getHistory();
    if (history.length > 1) {
      // Remove the last entry and go to the previous one
      // BrowsingHistoryManager does not expose removeLast, so we need to extend it if needed
      // For now, just simulate back navigation
      // (You may want to add a removeLast method to BrowsingHistoryManager for full support)
      // Example:
      // historyManager.removeLast();
      // final previousUrl = historyManager.getHistory().last;
      // _urlController.text = previousUrl;
      // _loadPage(previousUrl);
    }
  }

  void _clearPreviousPageData() {
    // Clear previous HTML/CSS to free memory
    setState(() {
      _currentHtml = '';
      _currentCss = '';
    });
    // Log memory usage for monitoring
    logMemoryUsage('After clearing previous page data', logPrint);
    // Force garbage collection if possible
    // Note: This is a hint to the garbage collector
    if (mounted) {
      // Trigger a rebuild to help with memory cleanup
      setState(() {});
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
      final errorMessage = 'Failed to initialize Rust engine:\n'
        '${result.errorMessage ?? ''}\n${result.stackTrace ?? ''}';
      _updateInitializationStatus(errorMessage, false);
      logPrint('[ERROR] browser_screen.dart:_initializeEngine: ${result.errorMessage}\n${result.stackTrace}');
      return;
    }
    _updateInitializationStatus('Rust engine initialized successfully', true);
  }

  void _handleInitializationError(dynamic error, StackTrace stack) {
    final errorMessage = 'Error initializing Rust engine: $error\n$stack';
    _updateInitializationStatus(errorMessage, false);
    logPrint('[ERROR] browser_screen.dart:_initializeEngine: $error\n$stack');
  }

  void _testJavaScript() {
    // Load the test JavaScript page
    _loadPage('file://${Directory.current.path}/test_javascript_website.html');
  }

  void _toggleDarkMode() {
    setState(() {
      _isDarkMode = !_isDarkMode;
    });
  }

  void _refreshPage() {
    if (_currentUrl.isNotEmpty) {
      _loadPage(_currentUrl);
    }
  }

  void _handleGoButtonPressed() {
    final url = _urlController.text;
    if (url.isNotEmpty) {
      _loadPage(url);
    }
  }

  void _onUrlChanged(String value) {
    _generateSuggestions(value);
  }

  void _handleUrlSubmission(String url) {
    if (url.isNotEmpty) {
      setState(() {
        _showSuggestions = false;
      });
      _loadPage(url);
    }
  }

  void _onUrlFieldTapped() {
    if (_urlController.text.isNotEmpty) {
      _generateSuggestions(_urlController.text);
    }
  }

  void _selectSuggestion(String suggestion) {
    _urlController.text = suggestion;
    setState(() {
      _showSuggestions = false;
    });
    _loadPage(suggestion);
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

  void _toggleDevConsole() {
    setState(() {
      _showDevConsole = !_showDevConsole;
    });
  }

  String _getScrollOffset() {
    return _scrollController.hasClients ? _scrollController.offset.toStringAsFixed(0) : "0";
  }

  double _getScrollOffsetValue() {
    return _scrollController.hasClients ? _scrollController.offset : 0.0;
  }

  void _updateLoadingProgress(double progress, String message) {
    setState(() {
      _loadingProgress = progress;
      _loadingMessage = message;
    });
  }

  Future<String> _extractCssFromHtml(String html) async {
    final styleRegex = RegExp(r'<style[^>]*>(.*?)</style>', dotAll: true);
    final matches = styleRegex.allMatches(html);
    String css = '';
    for (final match in matches) {
      css += match.group(1) ?? '';
    }
    return css;
  }

  List<String> _extractExternalCssUrls(String html, String baseUrl) {
    final linkRegex = RegExp(
      r'<link[^>]+rel="stylesheet"[^>]+href="([^"]+)"[^>]*>',
      caseSensitive: false);
    final matches = linkRegex.allMatches(html);
    final urls = <String>[];
    for (final match in matches) {
      final href = match.group(1);
      if (href != null && href.isNotEmpty) {
        urls.add(resolveUrl(href, baseUrl));
      }
    }
    return urls;
  }

  List<String> _extractImageUrls(String html, String css, String baseUrl) {
    final imgUrls = <String>[];
    final imgTagRegex = RegExp(
      r'<img[^>]+src="([^"]+)"[^>]*>',
      caseSensitive: false);
    for (final match in imgTagRegex.allMatches(html)) {
      final src = match.group(1);
      if (src != null && src.isNotEmpty) {
        imgUrls.add(resolveUrl(src, baseUrl));
      }
    }
    final bgImgRegex = RegExp(
      r'background-image\s*:\s*url\(([^)]+)\)',
      caseSensitive: false);
    for (final match in bgImgRegex.allMatches(css)) {
      final url = match.group(1);
      if (url != null && url.isNotEmpty) {
        imgUrls.add(resolveUrl(url, baseUrl));
      }
    }
    return imgUrls;
  }

  void _finalizeLoadingWithImages(String html, String css, List<String> imageUrls) {
    try {
      setState(() {
        _currentHtml = html;
        _currentCss = css;
        _isLoading = false;
        _loadingProgress = 1.0;
        _loadingMessage = 'Page loaded successfully!';
        _hasError = false;
        _errorMessage = '';
      });
      logPrint('[DART] _finalizeLoadingWithImages: Successfully parsed and loaded page');
    } catch (e) {
      logPrint('[DART] _finalizeLoadingWithImages: Error: $e');
      setState(() {
        _currentHtml = html;
        _currentCss = css;
        _isLoading = false;
        _loadingProgress = 1.0;
        _loadingMessage = 'Error loading page';
        _hasError = true;
        _errorMessage = 'Error loading page: $e';
      });
    }
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

  @override
  Widget build(BuildContext context) {
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
} 