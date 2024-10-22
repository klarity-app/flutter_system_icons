import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_system_icons/flutter_system_icons.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

class FileIconWidget extends HookConsumerWidget {
  final String filePath;
  final double radius;

  const FileIconWidget({
    super.key,
    required this.filePath,
    this.radius = 16.0,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final iconService = ref.watch(iconServicePOD);
    final iconFuture = useMemoized(() => iconService.getIconByPath(filePath), [filePath]);

    final snapshot = useFuture(iconFuture);

    if (snapshot.connectionState == ConnectionState.waiting) {
      return SizedBox(width: radius * 2, height: radius * 2);
    }

    if (snapshot.hasError || !snapshot.hasData || snapshot.data == null) {
      return SizedBox(width: radius * 2, height: radius * 2);
    }

    return _SystemIconWidget(
      systemIcon: snapshot.data!,
      radius: radius,
    );
  }
}

class DirectoryIconWidget extends HookConsumerWidget {
  final String directoryPath;
  final double radius;

  const DirectoryIconWidget({
    super.key,
    required this.directoryPath,
    this.radius = 16.0,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final iconService = ref.watch(iconServicePOD);
    final iconFuture = useMemoized(() => iconService.getIconByPath(directoryPath), [directoryPath]);

    final snapshot = useFuture(iconFuture);

    if (snapshot.connectionState == ConnectionState.waiting) {
      return SizedBox(width: radius * 2, height: radius * 2);
    }

    if (snapshot.hasError || !snapshot.hasData || snapshot.data == null) {
      return SizedBox(width: radius * 2, height: radius * 2);
    }

    return _SystemIconWidget(
      systemIcon: snapshot.data!,
      radius: radius,
    );
  }
}

class BundleIconWidget extends HookConsumerWidget {
  final String bundleIdentifier;
  final double radius;

  const BundleIconWidget({
    super.key,
    required this.bundleIdentifier,
    this.radius = 16.0,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final iconService = ref.watch(iconServicePOD);
    final iconFuture = useMemoized(() => iconService.getIconByBundleIdentifier(bundleIdentifier), [bundleIdentifier]);

    final snapshot = useFuture(iconFuture);

    if (snapshot.connectionState == ConnectionState.waiting) {
      return SizedBox(width: radius * 2, height: radius * 2);
    }

    if (snapshot.hasError || !snapshot.hasData || snapshot.data == null) {
      return SizedBox(width: radius * 2, height: radius * 2);
    }

    return _SystemIconWidget(
      systemIcon: snapshot.data!,
      radius: radius,
    );
  }
}

class _SystemIconWidget extends HookWidget {
  final SystemIcon systemIcon;
  final double radius;

  const _SystemIconWidget({
    required this.systemIcon,
    required this.radius,
  });

  @override
  Widget build(BuildContext context) {
    final iconBytes = useMemoized(() => base64Decode(systemIcon.data), [systemIcon]);

    return Image.memory(
      iconBytes,
      width: radius * 2,
      height: radius * 2,
      fit: BoxFit.cover,
    );
  }
}
