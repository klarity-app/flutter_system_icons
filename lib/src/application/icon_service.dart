import 'package:flutter_system_icons/flutter_system_icons.dart';
import 'package:flutter_system_icons/src/domain/repositories/icon_repository.dart';
import 'package:flutter_system_icons/src/infrastructure/icon_repository_impl.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'icon_service.g.dart';

@riverpod
IconService iconService(IconServiceRef ref) {
  return IconService(ref.watch(iconRepositoryPOD));
}

class IconService {
  final IconRepository _repository;

  IconService(this._repository);

  Future<SystemIcon?> getIconByBundleIdentifier(String bundleIdentifier) {
    return _repository.getIconByBundleIdentifier(bundleIdentifier);
  }

  Future<SystemIcon?> getIconByPath(String path) {
    return _repository.getIconByPath(path);
  }

  Future<List<SystemIcon?>> getMultipleIcons(List<String> identifiers) {
    return Future.wait(identifiers.map((identifier) => getIconByBundleIdentifier(identifier)));
  }
}
