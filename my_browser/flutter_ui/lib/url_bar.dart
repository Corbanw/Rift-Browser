import 'package:flutter/material.dart';

// TODO: Move suggestions and navigation logic to their own helpers if needed.

class UrlBar extends StatelessWidget {
  final TextEditingController urlController;
  final bool isDarkMode;
  final bool canGoBack;
  final bool canGoForward;
  final VoidCallback? onBack;
  final VoidCallback? onForward;
  final VoidCallback? onRefresh;
  final VoidCallback? onGo;
  final ValueChanged<String>? onChanged;
  final ValueChanged<String>? onSubmitted;
  final VoidCallback? onTap;
  final bool showSuggestions;
  final List<String> urlSuggestions;
  final Function(String)? onSuggestionSelected;

  const UrlBar({
    super.key,
    required this.urlController,
    required this.isDarkMode,
    required this.canGoBack,
    required this.canGoForward,
    this.onBack,
    this.onForward,
    this.onRefresh,
    this.onGo,
    this.onChanged,
    this.onSubmitted,
    this.onTap,
    this.showSuggestions = false,
    this.urlSuggestions = const [],
    this.onSuggestionSelected,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(12.0),
      decoration: BoxDecoration(
        color: isDarkMode ? Colors.grey[800] : Colors.grey[100],
        border: Border(
          bottom: BorderSide(
            color: isDarkMode ? Colors.grey[600]! : Colors.grey[300]!,
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
              IconButton(
                onPressed: canGoBack ? onBack : null,
                icon: Icon(
                  Icons.arrow_back,
                  color: canGoBack ? (isDarkMode ? Colors.white : Colors.blue) : Colors.grey,
                ),
                tooltip: 'Go back',
              ),
              IconButton(
                onPressed: canGoForward ? onForward : null,
                icon: Icon(
                  Icons.arrow_forward,
                  color: canGoForward ? (isDarkMode ? Colors.white : Colors.blue) : Colors.grey,
                ),
                tooltip: 'Go forward',
              ),
              IconButton(
                onPressed: onRefresh,
                icon: Icon(
                  Icons.refresh,
                  color: onRefresh != null ? (isDarkMode ? Colors.white : Colors.blue) : Colors.grey,
                ),
                tooltip: 'Refresh page',
              ),
              const SizedBox(width: 8),
              Expanded(
                child: Container(
                  decoration: BoxDecoration(
                    color: isDarkMode ? Colors.grey[700] : Colors.white,
                    borderRadius: BorderRadius.circular(8),
                    border: Border.all(
                      color: isDarkMode ? Colors.grey[600]! : Colors.grey[300]!,
                    ),
                  ),
                  child: TextField(
                    controller: urlController,
                    style: TextStyle(
                      color: isDarkMode ? Colors.white : Colors.black,
                      fontSize: 14,
                    ),
                    decoration: InputDecoration(
                      hintText: 'Enter URL or search...',
                      hintStyle: TextStyle(
                        color: isDarkMode ? Colors.grey[400] : Colors.grey[500],
                      ),
                      prefixIcon: Icon(
                        Icons.search,
                        color: isDarkMode ? Colors.grey[400] : Colors.grey[500],
                        size: 20,
                      ),
                      border: InputBorder.none,
                      contentPadding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
                    ),
                    onChanged: onChanged,
                    onSubmitted: onSubmitted,
                    onTap: onTap,
                  ),
                ),
              ),
              const SizedBox(width: 8),
              ElevatedButton(
                onPressed: onGo,
                child: const Text('Go'),
              ),
            ],
          ),
          if (showSuggestions && urlSuggestions.isNotEmpty)
            _buildUrlSuggestions(),
        ],
      ),
    );
  }

  Widget _buildUrlSuggestions() {
    return Container(
      margin: const EdgeInsets.only(top: 4),
      decoration: BoxDecoration(
        color: isDarkMode ? Colors.grey[800] : Colors.white,
        borderRadius: BorderRadius.circular(8),
        border: Border.all(
          color: isDarkMode ? Colors.grey[600]! : Colors.grey[300]!,
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
          itemCount: urlSuggestions.length,
          itemBuilder: (context, index) {
            final suggestion = urlSuggestions[index];
            return ListTile(
              dense: true,
              leading: const Icon(Icons.language, size: 16), // TODO: Use dynamic icons
              title: Text(
                suggestion,
                style: TextStyle(
                  fontSize: 13,
                  color: isDarkMode ? Colors.white : Colors.black,
                ),
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
              ),
              onTap: () => onSuggestionSelected?.call(suggestion),
            );
          },
        ),
      ),
    );
  }
} 