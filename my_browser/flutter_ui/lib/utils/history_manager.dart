class BrowsingHistoryManager {
  final List<String> _browsingHistory = [];
  final int maxHistoryLength;

  BrowsingHistoryManager({this.maxHistoryLength = 50});

  void addToHistory(String url) {
    String normalizedUrl = url;
    if (!url.startsWith('http')) {
      normalizedUrl = 'https://$url';
    }
    if (_browsingHistory.isEmpty || _browsingHistory.last != normalizedUrl) {
      _browsingHistory.add(normalizedUrl);
      if (_browsingHistory.length > maxHistoryLength) {
        _browsingHistory.removeAt(0);
      }
    }
  }

  List<String> getHistory() => List.unmodifiable(_browsingHistory);

  void clearHistory() => _browsingHistory.clear();
} 