import 'dart:js';
import 'engine_bridge.dart';

void setupJsDomAttributeBridge() {
  context['window']['_setAttribute'] = (int nodeId, String name, String value) {
    EngineBridge.setAttribute(nodeId, name, value);
  };
  context['window']['_getAttribute'] = (int nodeId, String name) {
    return EngineBridge.getAttribute(nodeId, name);
  };
  context['window']['_removeAttribute'] = (int nodeId, String name) {
    EngineBridge.removeAttribute(nodeId, name);
  };
  context['window']['_hasAttribute'] = (int nodeId, String name) {
    return EngineBridge.hasAttribute(nodeId, name);
  };
  context['window']['_classListAdd'] = (int nodeId, String className) {
    EngineBridge.classListAdd(nodeId, className);
  };
  context['window']['_classListRemove'] = (int nodeId, String className) {
    EngineBridge.classListRemove(nodeId, className);
  };
  context['window']['_classListToggle'] = (int nodeId, String className) {
    EngineBridge.classListToggle(nodeId, className);
  };
  context['window']['_classListContains'] = (int nodeId, String className) {
    return EngineBridge.classListContains(nodeId, className);
  };
  context['window']['_getTextContent'] = (int nodeId) {
    return EngineBridge.getTextContent(nodeId);
  };
  context['window']['_setTextContent'] = (int nodeId, String value) {
    EngineBridge.setTextContent(nodeId, value);
  };
  context['window']['_getInnerHtml'] = (int nodeId) {
    return EngineBridge.getInnerHtml(nodeId);
  };
  context['window']['_setInnerHtml'] = (int nodeId, String value) {
    EngineBridge.setInnerHtml(nodeId, value);
  };
  context['window']['_getOuterHtml'] = (int nodeId) {
    return EngineBridge.getOuterHtml(nodeId);
  };
  context['window']['_setOuterHtml'] = (int nodeId, String value) {
    EngineBridge.setOuterHtml(nodeId, value);
  };
  context['window']['_getId'] = (int nodeId) {
    return EngineBridge.getId(nodeId);
  };
  context['window']['_setId'] = (int nodeId, String value) {
    EngineBridge.setId(nodeId, value);
  };
  context['window']['_getTagName'] = (int nodeId) {
    return EngineBridge.getTagName(nodeId);
  };
  context['window']['_getNodeType'] = (int nodeId) {
    return EngineBridge.getNodeType(nodeId);
  };
  context['window']['_getStyle'] = (int nodeId, String name) {
    return EngineBridge.getStyle(nodeId, name);
  };
  context['window']['_setStyle'] = (int nodeId, String name, String value) {
    EngineBridge.setStyle(nodeId, name, value);
  };
  context['window']['_getStyleCssText'] = (int nodeId) {
    return EngineBridge.getStyleCssText(nodeId);
  };
  context['window']['_setStyleCssText'] = (int nodeId, String value) {
    EngineBridge.setStyleCssText(nodeId, value);
  };
  context['window']['_addEventListener'] = (int nodeId, String type, int callbackId) {
    EngineBridge.addEventListener(nodeId, type, callbackId);
  };
  context['window']['_removeEventListener'] = (int nodeId, String type, int callbackId) {
    EngineBridge.removeEventListener(nodeId, type, callbackId);
  };
  context['window']['_dispatchEvent'] = (int nodeId, String type) {
    EngineBridge.dispatchEvent(nodeId, type);
  };
} 