import 'package:client/features/authentification/screens/login/login.dart';
import 'package:flutter/material.dart';
import 'package:client/utils/theme/theme.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

void main() {
  final FlutterSecureStorage secureStorage = FlutterSecureStorage();
  runApp(App(secureStorage: secureStorage));
}

class App extends StatelessWidget {
  final FlutterSecureStorage secureStorage;
  const App({required this.secureStorage});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      themeMode: ThemeMode.system,
      theme: TAppTheme.lightTheme,
      darkTheme: TAppTheme.darkTheme,
      home: LoginScreen(),
    );
  }
}