import 'package:flutter_system_icons/flutter_system_icons.dart';

abstract class IconRepository {
  Future<SystemIcon?> getIconByBundleIdentifier(String bundleIdentifier);
  Future<SystemIcon?> getIconByPath(String path);
}
