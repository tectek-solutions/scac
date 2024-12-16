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
  final _formKey = GlobalKey<FormState>();
  final _nameController = TextEditingController();
  final _firstnameController = TextEditingController();
  final _emailController = TextEditingController();
  final _passwordController = TextEditingController();
  final ApiAccountService _apiService = ApiAccountService(baseUrl: 'https://yourapi.com');

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
                          _buildInputField(controller: _firstnameController, label: "First Name", icon: Icons.person),
                          const SizedBox(height: TSizes.spaceBtwItemsInputFields),
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
                      //HERE
                      onPressed: () async {
                        if (_formKey.currentState!.validate()) {
                          final name = _nameController.text;
                          final firstName = _firstnameController.text;
                          final email = _emailController.text;
                          final password = _passwordController.text;

                          print("Name: " + name + "FirstName: " + firstName + "Email: " + email + "Password" + password);

                          // try {
                          //   final response = await _apiService.signUp(name, firstName, email, password);
                          //   if (response["isSuccessful"]) {
                          //     // Handle successful sign-up
                          //     ScaffoldMessenger.of(context).showSnackBar(
                          //       SnackBar(content: Text('Sign-up successful!')),
                          //     );
                          //     // Navigate to another screen if needed
                          //   } else {
                          //     // Handle sign-up error
                          //     ScaffoldMessenger.of(context).showSnackBar(
                          //       SnackBar(content: Text('Sign-up failed: ${response["errorMessage"]}')),
                          //     );
                          //   }
                          // } catch (e) {
                          //   // Handle any other errors
                          //   ScaffoldMessenger.of(context).showSnackBar(
                          //     SnackBar(content: Text('An error occurred: $e')),
                          //   );
                          // }
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
