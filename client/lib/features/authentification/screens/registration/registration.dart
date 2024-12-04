import 'package:client/common/style/spacing_styles.dart';
import 'package:client/features/authentification/screens/login/login.dart';
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
  final TextEditingController _passwordController = TextEditingController();

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
                    child: Padding(
                      padding: const EdgeInsets.symmetric(
                          vertical: TSizes.sapceBtwSections),
                      child: Column(
                        children: [
                          _buildInputField(label: "Name", icon: Icons.person),
                          const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                          _buildInputField(label: "First Name", icon: Icons.person),
                          const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                          _buildInputField(label: "Email", icon: Icons.email),
                          const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                          _buildPasswordInputField(),
                        ],
                      ),
                    ),
                  ),

                  SizedBox(
                    width: MediaQuery.of(context).size.width,
                    child: OutlinedButton(
                      onPressed: () {},
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

  Widget _buildInputField({required String label, required IconData icon}) {
    return TextFormField(
      decoration: InputDecoration(
        labelText: label,
        prefixIcon: Icon(icon),
      ),
    );
  }

  Widget _buildPasswordInputField() {
    return TextFormField(
      controller: _passwordController,
      obscureText: _isObscured,
      decoration: InputDecoration(
        labelText: "Password",
        prefixIcon: const Icon(Icons.password),
        suffixIcon: IconButton(
          icon: _isObscured
              ? const Icon(Icons.visibility_off)
              : const Icon(Icons.visibility),
          onPressed: () {
            setState(() {
              _isObscured = !_isObscured;
            });
          },
        ),
      ),
    );
  }
}
