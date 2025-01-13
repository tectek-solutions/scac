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
  final FlutterSecureStorage secureStorage;

  LoginScreen({required this.secureStorage});

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

  final String _url_google = "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id=936038757007-d2vvj4kjm98vcod9e9ek9ilvoeij1fcr.apps.googleusercontent.com&redirect_uri=$baseUrlString/oauth2/authorize/google&scope=https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email https://www.googleapis.com/auth/calendar https://mail.google.com/&state=test_state";

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
                              final token = response;
                              await widget.secureStorage.write(key: 'auth_token', value: token);
                              Map<String, String> allValues = await widget.secureStorage.readAll();
                              print(allValues);
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
                        // Navigator.push(
                        //   context,
                        //   MaterialPageRoute(
                        //     builder: (context) => const Registration(),
                        //   ),
                        // );
                      },
                      child: const Text("Create an account"),
                    ),
                  ),
                  const SizedBox(height: TSizes.spaceBtwItems),
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Flexible(
                        child: Divider(
                        color: dark
                            ? Colors.grey.shade700
                            : Colors.grey.shade300,
                        thickness: 1.5,
                        indent: 60,
                        endIndent: 5,
                      )),
                      const Text(
                        "Or Sign In With",
                        style: TextStyle(color: Colors.grey),
                      ),
                      Flexible(
                          child: Divider(
                        color: dark
                            ? Colors.grey.shade700
                            : Colors.grey.shade300,
                        thickness: 1.5,
                        indent: 5,
                        endIndent: 60,
                      )),
                    ],
                  ),

                  const SizedBox(height: TSizes.sapceBtwSections),

                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Container(
                        decoration: BoxDecoration(
                          border: Border.all(color: Colors.grey),
                          borderRadius: BorderRadius.circular(100),
                        ),
                        child: IconButton(
                          onPressed: () async {
                            try {
                              await launchUrl(Uri.parse(_url_google));
                            } catch (e) {
                              print('Erreur lors de l\'ouverture du lien: $e');
                            }
                          },
                          icon: const Image(
                            width: TSizes.iconMd,
                            height: TSizes.iconMd,
                            image: AssetImage(TImages.google),
                          ),
                        ),
                      ),
                      const SizedBox(width: TSizes.spaceBtwItems),
                      Container(
                        decoration: BoxDecoration(
                          border: Border.all(color: Colors.grey),
                          borderRadius: BorderRadius.circular(100),
                        ),
                        child: IconButton(
                          onPressed: () {},
                          icon: const Image(
                            width: TSizes.iconMd,
                            height: TSizes.iconMd,
                            image: AssetImage(TImages.facebook),
                          ),
                        ),
                      ),
                    ],
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