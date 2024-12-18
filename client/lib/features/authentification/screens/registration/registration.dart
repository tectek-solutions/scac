import 'package:client/common/style/spacing_styles.dart';
import 'package:client/features/authentification/screens/login/login.dart';
import 'package:client/features/authentification/services/api.service.dart';
import 'package:flutter/material.dart';
import '../../../../utils/constants/helper_functions.dart';
import 'package:client/utils/constants/image_strings.dart';
import '../../../../utils/constants/sizes.dart';
import 'package:client/utils/constants/text_string.dart';

class Registration extends StatefulWidget {
  const Registration({super.key});

  @override
  // ignore: library_private_types_in_public_api
  _RegistrationState createState() => _RegistrationState();
}

class _RegistrationState extends State<Registration> {
  var _isObscured = true;
  static const baseUrlString = String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');
  final _formKey = GlobalKey<FormState>();
  final _nameController = TextEditingController();
  final _password_confirmationController = TextEditingController();
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
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
                        TText.registrationTitle,
                        style: Theme.of(context).textTheme.headlineMedium,
                      ),
                      const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                    ],
                  ),
                  
                  Form(
                    key: _formKey,
                    child: Padding(
                      padding: const EdgeInsets.symmetric(
                          vertical: TSizes.sapceBtwSections),
                      child: Column(
                        children: [
                          _buildInputField(controller: _nameController, label: "Name", icon: Icons.person),
                          const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                          _buildInputField(controller: _emailController, label: "Email", icon: Icons.email),
                          const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                          _buildPasswordInputField(controller: _passwordController, label: "Password"),
                          const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                          _buildPasswordInputField(controller: _password_confirmationController, label: "Password confirmation"),
                        ],
                      ),
                    ),
                  ),

                  SizedBox(
                    width: MediaQuery.of(context).size.width,
                    child: OutlinedButton(
                      //HERE
                      onPressed: () async {
                        if (_formKey.currentState!.validate()) {
                          final name = _nameController.text;
                          final email = _emailController.text;
                          final password = _passwordController.text;
                          final password_confirmation = _password_confirmationController.text;
                          try {
                            await _apiService.signUp(name, email, password, password_confirmation);
                            Navigator.push(
                              context,
                              MaterialPageRoute(builder: (context) => const LoginScreen()),
                            );
                          } catch (e) {
                            ScaffoldMessenger.of(context).showSnackBar(
                              SnackBar(
                                content: Text(e.toString()),
                              ),
                            );
                          }
                        }
                      },
                      style: ElevatedButton.styleFrom(
                        backgroundColor: Colors.blue,
                      ),
                      child: const Text("Sign up"),
                    ),
                  ),

                  const SizedBox(height: TSizes.sapceBtwSections),

                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Flexible(
                          child: Divider(
                        color: dark ? Colors.grey.shade700 : Colors.grey.shade300,
                        thickness: 1.5,
                        indent: 60,
                        endIndent: 5,
                      )),
                      const Text("Already have an account ?", style: TextStyle(color: Colors.grey)),
                      Flexible(
                          child: Divider(
                        color: dark ? Colors.grey.shade700 : Colors.grey.shade300,
                        thickness: 1.5,
                        indent: 5,
                        endIndent: 60,
                      )),
                    ],
                  ),

                  const SizedBox(height: TSizes.sapceBtwSections),

                  SizedBox(
                    width: MediaQuery.of(context).size.width,
                    child: OutlinedButton.icon(
                      onPressed: () {
                        Navigator.push(
                          context,
                          MaterialPageRoute(builder: (context) => const LoginScreen()),
                        );
                      },
                      style: ElevatedButton.styleFrom(
                        backgroundColor: Colors.transparent,
                      ),
                      icon: const Icon(Icons.arrow_back),
                      label: const Text("Go back"),
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

  Widget _buildPasswordInputField({required TextEditingController controller, required String label}) {
    return TextFormField(
      controller: controller,
      obscureText: _isObscured,
      decoration: InputDecoration(
        labelText: label,
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
