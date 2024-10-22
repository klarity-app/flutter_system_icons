import 'package:flutter/material.dart';
import 'package:flutter_system_icons/flutter_system_icons.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

void main() {
  runApp(const ProviderScope(child: MyApp()));
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter System Icons Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      debugShowCheckedModeBanner: false,
      home: const MyHomePage(title: 'Flutter System Icons Demo'),
    );
  }
}

class MyHomePage extends StatelessWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              Padding(
                padding: const EdgeInsets.all(16),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('Flutter System Icons Demo',
                        style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold)),
                    const SizedBox(height: 24),
                    _buildIconShowcase(
                      title: 'File Icon',
                      explanation:
                          'This displays the icon of a specific file on your macOS system. In this example, we\'re showing the icon for the Finder application.',
                      icon: const FileIconWidget(
                        filePath: '/System/Library/CoreServices/Finder.app',
                        radius: 24.0,
                      ),
                    ),
                    const SizedBox(height: 24),
                    _buildIconShowcase(
                      title: 'Directory Icon',
                      explanation:
                          'This shows the icon of a directory on your macOS system. Here, we\'re displaying the icon for the Applications folder.',
                      icon: const DirectoryIconWidget(
                        directoryPath: '/Applications',
                        radius: 24.0,
                      ),
                    ),
                    const SizedBox(height: 24),
                    _buildIconShowcase(
                      title: 'Application Icon (macOS only)',
                      explanation:
                          'This retrieves and displays the icon of a macOS application using its bundle identifier. In this case, we\'re showing the Safari browser icon.',
                      icon: const BundleIconWidget(
                        bundleIdentifier: 'com.apple.Safari',
                        radius: 32.0,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildIconShowcase({
    required String title,
    required String explanation,
    required Widget icon,
  }) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(title, style: const TextStyle(fontSize: 18, fontWeight: FontWeight.bold)),
        const SizedBox(height: 8),
        Text(explanation, style: const TextStyle(fontSize: 14, color: Colors.grey)),
        const SizedBox(height: 16),
        Center(child: icon),
      ],
    );
  }
}
