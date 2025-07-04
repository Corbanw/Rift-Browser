import 'dart:ffi' as ffi;
import 'ffi_structs.dart';

// Type aliases for compatibility
// Only Char and Void are valid in dart:ffi

typedef Void = ffi.Void;
typedef Char = ffi.Char;

// Layout and draw command typedefs
typedef ParseHtmlC = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>);
typedef GetDrawCommandCountC = ffi.Int32 Function(ffi.Pointer<ffi.Void>);
typedef GetDrawCommandC = ffi.Pointer<FFIDrawCommand> Function(ffi.Pointer<ffi.Void>, ffi.Int32);
typedef FreeDrawCommandArrayC = ffi.Void Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxCountC = ffi.Int32 Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxC = ffi.Pointer<FFILayoutBox> Function(ffi.Pointer<ffi.Void>, ffi.Int32);
typedef GetLayoutBoxXC = ffi.Float Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxYC = ffi.Float Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxWidthC = ffi.Float Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxHeightC = ffi.Float Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxNodeTypeC = ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxTextContentC = ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxBackgroundColorC = ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxColorC = ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxFontSizeC = ffi.Float Function(ffi.Pointer<ffi.Void>);
typedef FreeLayoutBoxArrayC = ffi.Void Function(ffi.Pointer<ffi.Void>);
typedef FreeLayoutBoxC = ffi.Void Function(ffi.Pointer<ffi.Void>);
typedef FreeCStringC = ffi.Void Function(ffi.Pointer<ffi.Char>);

// Dart signatures
// ... existing code ...
typedef ParseHtmlDart = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>);
typedef GetDrawCommandCountDart = int Function(ffi.Pointer<ffi.Void>);
typedef GetDrawCommandDart = ffi.Pointer<FFIDrawCommand> Function(ffi.Pointer<ffi.Void>, int);
typedef FreeDrawCommandArrayDart = void Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxCountDart = int Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxDart = ffi.Pointer<FFILayoutBox> Function(ffi.Pointer<ffi.Void>, int);
typedef GetLayoutBoxXDart = double Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxYDart = double Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxWidthDart = double Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxHeightDart = double Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxNodeTypeDart = ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxTextContentDart = ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxBackgroundColorDart = ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxColorDart = ffi.Pointer<ffi.Char> Function(ffi.Pointer<ffi.Void>);
typedef GetLayoutBoxFontSizeDart = double Function(ffi.Pointer<ffi.Void>);
typedef FreeLayoutBoxArrayDart = void Function(ffi.Pointer<ffi.Void>);
typedef FreeLayoutBoxDart = void Function(ffi.Pointer<ffi.Void>);
typedef FreeCStringDart = void Function(ffi.Pointer<ffi.Char>);

// ... (continue for all other typedefs, grouped by feature: DOM, style, events, JS, etc.) 

// Enhanced/batch/other FFI typedefs
typedef GetLayoutBoxBatchC = ffi.Int32 Function(ffi.Pointer<ffi.Void>, ffi.Int32, ffi.Int32, ffi.Pointer<ffi.Pointer<FFILayoutBox>>);
typedef GetLayoutBoxBatchDart = int Function(ffi.Pointer<ffi.Void>, int, int, ffi.Pointer<ffi.Pointer<FFILayoutBox>>);
typedef GetLayoutBoxBatchEnhancedC = ffi.Int32 Function(ffi.Pointer<ffi.Void>, ffi.Int32, ffi.Int32, ffi.Pointer<ffi.Pointer<FFILayoutBox>>);
typedef GetLayoutBoxBatchEnhancedDart = int Function(ffi.Pointer<ffi.Void>, int, int, ffi.Pointer<ffi.Pointer<FFILayoutBox>>);
typedef ParseHtmlToDrawCommandsC = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>);
typedef ParseHtmlToDrawCommandsDart = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>);
typedef ParseUrlViaRustC = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>);
typedef ParseUrlViaRustDart = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>);
typedef ParseUrlViaRustEnhancedC = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>);
typedef ParseUrlViaRustEnhancedDart = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>);

typedef ExecuteJavaScriptC = ffi.Int32 Function(ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>);
typedef ExecuteJavaScriptDart = int Function(ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>);
typedef ParseHtmlWithJavaScriptC = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>);
typedef ParseHtmlWithJavaScriptDart = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>);

typedef ParseHtmlWithCssC = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>);
typedef ParseHtmlWithCssDart = ffi.Pointer<ffi.Void> Function(ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>);

typedef FreeFFILayoutBoxC = ffi.Void Function(ffi.Pointer<FFILayoutBox>);
typedef FreeFFILayoutBoxDart = void Function(ffi.Pointer<FFILayoutBox>);

// DOM attribute FFI typedefs
typedef DomGetAttributeC = ffi.Pointer<ffi.Char> Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomSetAttributeC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>);
typedef DomRemoveAttributeC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomHasAttributeC = ffi.Uint8 Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomGetAttributeDart = ffi.Pointer<ffi.Char> Function(int, ffi.Pointer<ffi.Char>);
typedef DomSetAttributeDart = void Function(int, ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>);
typedef DomRemoveAttributeDart = void Function(int, ffi.Pointer<ffi.Char>);
typedef DomHasAttributeDart = int Function(int, ffi.Pointer<ffi.Char>);

// classList FFI typedefs
typedef DomClassListAddC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomClassListRemoveC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomClassListToggleC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomClassListContainsC = ffi.Uint8 Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomClassListAddDart = void Function(int, ffi.Pointer<ffi.Char>);
typedef DomClassListRemoveDart = void Function(int, ffi.Pointer<ffi.Char>);
typedef DomClassListToggleDart = void Function(int, ffi.Pointer<ffi.Char>);
typedef DomClassListContainsDart = int Function(int, ffi.Pointer<ffi.Char>);

// Node/Element property FFI typedefs
typedef DomGetTextContentC = ffi.Pointer<ffi.Char> Function(ffi.Uint32);
typedef DomSetTextContentC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomGetInnerHtmlC = ffi.Pointer<ffi.Char> Function(ffi.Uint32);
typedef DomSetInnerHtmlC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomGetOuterHtmlC = ffi.Pointer<ffi.Char> Function(ffi.Uint32);
typedef DomSetOuterHtmlC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomGetIdC = ffi.Pointer<ffi.Char> Function(ffi.Uint32);
typedef DomSetIdC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomGetTagNameC = ffi.Pointer<ffi.Char> Function(ffi.Uint32);
typedef DomGetNodeTypeC = ffi.Int32 Function(ffi.Uint32);
typedef DomGetTextContentDart = ffi.Pointer<ffi.Char> Function(int);
typedef DomSetTextContentDart = void Function(int, ffi.Pointer<ffi.Char>);
typedef DomGetInnerHtmlDart = ffi.Pointer<ffi.Char> Function(int);
typedef DomSetInnerHtmlDart = void Function(int, ffi.Pointer<ffi.Char>);
typedef DomGetOuterHtmlDart = ffi.Pointer<ffi.Char> Function(int);
typedef DomSetOuterHtmlDart = void Function(int, ffi.Pointer<ffi.Char>);
typedef DomGetIdDart = ffi.Pointer<ffi.Char> Function(int);
typedef DomSetIdDart = void Function(int, ffi.Pointer<ffi.Char>);
typedef DomGetTagNameDart = ffi.Pointer<ffi.Char> Function(int);
typedef DomGetNodeTypeDart = int Function(int);

// Style API FFI typedefs
typedef DomGetStyleC = ffi.Pointer<ffi.Char> Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomSetStyleC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>);
typedef DomGetStyleCssTextC = ffi.Pointer<ffi.Char> Function(ffi.Uint32);
typedef DomSetStyleCssTextC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomGetStyleDart = ffi.Pointer<ffi.Char> Function(int, ffi.Pointer<ffi.Char>);
typedef DomSetStyleDart = void Function(int, ffi.Pointer<ffi.Char>, ffi.Pointer<ffi.Char>);
typedef DomGetStyleCssTextDart = ffi.Pointer<ffi.Char> Function(int);
typedef DomSetStyleCssTextDart = void Function(int, ffi.Pointer<ffi.Char>);

// Event Handling API FFI typedefs
typedef DomAddEventListenerC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>, ffi.Uint32);
typedef DomRemoveEventListenerC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>, ffi.Uint32);
typedef DomDispatchEventC = ffi.Void Function(ffi.Uint32, ffi.Pointer<ffi.Char>);
typedef DomAddEventListenerDart = void Function(int, ffi.Pointer<ffi.Char>, int);
typedef DomRemoveEventListenerDart = void Function(int, ffi.Pointer<ffi.Char>, int);
typedef DomDispatchEventDart = void Function(int, ffi.Pointer<ffi.Char>); 