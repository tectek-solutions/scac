import 'package:client/features/authentification/screens/login/login.dart';
import 'package:flutter/material.dart';
import 'package:client/utils/theme/theme.dart';

Future<void> main() async {
  runApp(const App());
}

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    // String? myEnvVar = Platform.environment['SOME_VAR'];
    const SOME_VAR = String.fromEnvironment('SOME_VAR', defaultValue: 'SOME_DEFAULT_VALUE');
    print('MY_ENV_VAR: $SOME_VAR');
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      themeMode: ThemeMode.system,
      theme: TAppTheme.lightTheme,
      darkTheme: TAppTheme.darkTheme,
      home: const LoginScreen(),
    );
  }
}