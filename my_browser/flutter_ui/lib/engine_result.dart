class EngineInitResult {
  final bool success;
  final String? errorMessage;
  final String? stackTrace;
  EngineInitResult.success() : success = true, errorMessage = null, stackTrace = null;
  EngineInitResult.failure(this.errorMessage, this.stackTrace) : success = false;
} 