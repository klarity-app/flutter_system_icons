import 'package:flutter/foundation.dart';
import 'package:flutter_system_icons/src/domain/repositories/icon_repository.dart';
import 'package:flutter_system_icons/src/generated/frb_generated.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../generated/api/common.dart';

part 'icon_repository_impl.g.dart';

@riverpod
IconRepository iconRepository(IconRepositoryRef ref) {
  return IconRepositoryImpl();
}

class IconRepositoryImpl implements IconRepository {
  static bool _isInitialized = false;
  static final Future<void> _initFuture = _initializeOnce();

  static Future<void> _initializeOnce() async {
    if (!_isInitialized) {
      await RustLib.init();
      _isInitialized = true;
    }
  }

  Future<void> _ensureInitialized() async {
    await _initFuture;
  }

  @override
  Future<SystemIcon?> getIconByBundleIdentifier(String bundleIdentifier) async {
    await _ensureInitialized();
    try {
      return SystemIconService.getIconForBundle(bundleIdentifier: bundleIdentifier);
    } catch (e) {
      debugPrint('Error finding icon by bundle identifier: $e');
      return null;
    }
  }

  @override
  Future<SystemIcon?> getIconByPath(String path) async {
    await _ensureInitialized();
    try {
      return SystemIconService.getIconForFile(path: path);
    } catch (e) {
      debugPrint('Error finding icon by path: $e');
      return null;
    }
  }
}
