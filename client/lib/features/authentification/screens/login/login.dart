import 'package:client/features/authentification/screens/registration/registration.dart';
import 'package:client/utils/constants/image_strings.dart';
import 'package:client/utils/constants/text_string.dart';
import 'package:flutter/material.dart';
import '../../../../common/style/spacing_styles.dart';
import '../../../../utils/constants/helper_functions.dart';
import '../../../../utils/constants/sizes.dart';

class LoginScreen extends StatefulWidget {
  const LoginScreen({super.key});

  @override
  // ignore: library_private_types_in_public_api
  _LoginScreen createState() => _LoginScreen();
}

class _LoginScreen extends State<LoginScreen> {

  var _isObscured = false;
  final TextEditingController _passwordController = TextEditingController();

  @override
  Widget build(BuildContext context){
    final dark = THelperFunctions.isDarkMode(context);
    return Scaffold(
      body: SingleChildScrollView(
        child: Padding(
          padding: TSpacingStyles.defaultPaddingWithAppBarHeight,
          child: Column(
            children: [
              Column(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  Image(
                    height: 150,
                    image: AssetImage(dark ? TImages.darkAppLogo: TImages.ligthAppLogo),
                  ),
                  Text(TText.loginTitle, style: Theme.of(context).textTheme.headlineMedium),
                  const SizedBox(height: TSizes.sm),
                  Text(TText.loginSubtitle, style: Theme.of(context).textTheme.bodyMedium),
                ],
              ),

              Form(child: Padding(
                padding: const EdgeInsets.symmetric(vertical: TSizes.sapceBtwSections),
                child: Column(
                  children: [
                
                    TextFormField(
                      decoration: const InputDecoration(prefixIcon: Icon(Icons.email), labelText: "Email",
                      ),
                    ),
                
                    const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                
                    TextFormField(
                        controller: _passwordController,
                        obscureText: _isObscured,
                        decoration: InputDecoration(
                          labelText: "Password",
                          prefixIcon: const Icon(Icons.password),
                          suffixIcon: IconButton(
                            icon: _isObscured ? const Icon(Icons.visibility_off) : const Icon(Icons.visibility),
                            onPressed: () {
                              setState(() {
                                _isObscured = !_isObscured;
                              });
                            },
                          ),
                        ),
                      ),
                
                    const SizedBox(height: TSizes.spaceBtwItemsInputFields / 2),
                    const SizedBox(height: TSizes.sapceBtwSections),
                
                    SizedBox(width: double.infinity, child: OutlinedButton(onPressed: (){}, child: const Text("Sign in"), style: ElevatedButton.styleFrom(
                      backgroundColor: Colors.blue,
                    ),)),
                    const SizedBox(height: TSizes.spaceBtwItems),
                    SizedBox(width: double.infinity, child: OutlinedButton(onPressed: (){Navigator.push(context, MaterialPageRoute(builder: (constext) => Registration()));}, child: const Text("Create an account"))),
                
                    const SizedBox(height: TSizes.sapceBtwSections),
                  ],
                ),
              ),
              ),
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Flexible(child: Divider(color: dark ? Colors.grey.shade700 : Colors.grey.shade300, thickness: 1.5, indent: 60, endIndent: 5)),
                  const Text("Or Sign In With", style: TextStyle(color: Colors.grey)),
                  Flexible(child: Divider(color: dark ? Colors.grey.shade700 : Colors.grey.shade300, thickness: 1.5, indent: 5, endIndent: 60)),
                ],
              ),

              const SizedBox(height: TSizes.sapceBtwSections),

              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Container(
                    decoration: BoxDecoration(border: Border.all(color: Colors.grey), borderRadius: BorderRadius.circular(100)),
                    child: IconButton(
                      onPressed: () {},
                      icon: const Image(
                        width: TSizes.iconMd,
                        height: TSizes.iconMd,
                        image: AssetImage(TImages.google),
                      ),
                    ),
                  ),

                  const SizedBox(width: TSizes.spaceBtwItems),

                  Container(
                    decoration: BoxDecoration(border: Border.all(color: Colors.grey), borderRadius: BorderRadius.circular(100)),
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
              )
            ],
          ),
        ),
      ),
    );
  }
}