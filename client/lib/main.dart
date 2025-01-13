import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:client/features/authentification/screens/login/login.dart';
import 'package:client/utils/theme/theme.dart';
import 'package:client/features/authentification/screens/main-screen/main-screen.dart';
import 'package:jwt_decoder/jwt_decoder.dart';
import 'package:client/features/authentification/services/api.service.dart';


void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  bool isExpired = await isTokenExpired();
  runApp(App(isTokenExpired: isExpired));
}

class App extends StatelessWidget {
  final bool isTokenExpired;

  const App({super.key, required this.isTokenExpired});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      themeMode: ThemeMode.system,
      theme: TAppTheme.lightTheme,
      darkTheme: TAppTheme.darkTheme,
      home: isTokenExpired ? LoginScreen(secureStorage:const FlutterSecureStorage()) : const MainScreen(),
    );
  }
}

