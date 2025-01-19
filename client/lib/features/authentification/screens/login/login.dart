import 'package:client/features/authentification/screens/registration/registration.dart';
import 'package:client/utils/constants/image_strings.dart';
import 'package:client/utils/constants/text_string.dart';
import 'package:flutter/material.dart';
import '../../../../common/style/spacing_styles.dart';
import '../../../../utils/constants/helper_functions.dart';
import '../../../../utils/constants/sizes.dart';
import '../../../app/screens/main-screen/main-screen.dart';
import 'package:client/features/services/api.service.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class LoginScreen extends StatefulWidget {
  const LoginScreen({super.key});

  @override
  _LoginScreen createState() => _LoginScreen();
}

class _LoginScreen extends State<LoginScreen> {
  var _isObscured = true;
  final _formKey = GlobalKey<FormState>();
  static const baseUrlString = String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');
  final TextEditingController _emailController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  final ApiAccountService _apiService = ApiAccountService(baseUrl: baseUrlString);

  @override
  Widget build(BuildContext context) {
    final dark = THelperFunctions.isDarkMode(context);
    return Scaffold(
      body: Center(
        child: Padding(
          padding: TSpacingStyles.defaultPaddingWithAppBarHeight,
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 800),
            child: SingleChildScrollView(
              child: Column(
                children: [
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      Image(
                        height: 150,
                        image: AssetImage(
                          dark ? TImages.darkAppLogo : TImages.ligthAppLogo,
                        ),
                      ),
                      Text(
                        TText.loginTitle,
                        style: Theme.of(context).textTheme.headlineMedium,
                      ),
                      const SizedBox(height: TSizes.sm),
                      Text(
                        TText.loginSubtitle,
                        style: Theme.of(context).textTheme.bodyMedium,
                      ),
                    ],
                  ),
                  Form(
                    key: _formKey,
                    child: Padding(
                      padding: const EdgeInsets.symmetric(vertical: TSizes.sapceBtwSections),
                      child: Column(
                        children: [
                          _buildInputField(controller: _emailController, label: "Email", icon: Icons.email),
                          const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                          _buildPasswordInputField(controller: _passwordController),
                        ],
                      ),
                    ),
                  ),
                  SizedBox(
                    width: MediaQuery.of(context).size.width,
                    child: OutlinedButton(
                     onPressed: () async {
                        if (_formKey.currentState!.validate()) {
                          final email = _emailController.text;
                          final password = _passwordController.text;

                          try {
                            final response = await _apiService.signIn(email, password);
                            if (response is String) {
                              Navigator.push(
                                context,
                                MaterialPageRoute(
                                  builder: (context) => const MainScreen(),
                                ),
                              );
                            } else {
                              print("Unexpected response type: ${response.runtimeType}");
                            }
                          } catch (e) {
                            print("Error: $e");
                          }
                        }
                      },
                      style: ElevatedButton.styleFrom(
                        backgroundColor: Colors.blue,
                      ),
                      child: const Text("Sign in"),
                    ),
                  ),
                  const SizedBox(height: TSizes.spaceBtwItems),
                  SizedBox(
                    width: double.infinity,
                    child: OutlinedButton(
                      onPressed: () {
                        Navigator.push(
                          context,
                          MaterialPageRoute(
                            builder: (context) => const Registration(),
                          ),
                        );
                      },
                      child: const Text("Create an account"),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildInputField({required TextEditingController controller, required String label, required IconData icon}) {
    return TextFormField(
      controller: controller,
      decoration: InputDecoration(
        labelText: label,
        prefixIcon: Icon(icon),
      ),
      validator: (value) {
        if (value == null || value.isEmpty) {
          return 'Please enter $label';
        }
        return null;
      },
    );
  }

  Widget _buildPasswordInputField({required TextEditingController controller}) {
    return TextFormField(
      controller: controller,
      obscureText: _isObscured,
      decoration: InputDecoration(
        labelText: 'Password',
        prefixIcon: Icon(Icons.lock),
        suffixIcon: IconButton(
          icon: Icon(_isObscured ? Icons.visibility : Icons.visibility_off),
          onPressed: () {
            setState(() {
              _isObscured = !_isObscured;
            });
          },
        ),
      ),
      validator: (value) {
        if (value == null || value.isEmpty) {
          return 'Please enter password';
        }
        return null;
      },
    );
  }
}