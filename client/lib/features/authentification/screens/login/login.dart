import 'package:client/features/authentification/screens/registration/registration.dart';
import 'package:client/utils/constants/image_strings.dart';
import 'package:client/utils/constants/text_string.dart';
import 'package:flutter/material.dart';
import '../../../../common/style/spacing_styles.dart';
import '../../../../utils/constants/helper_functions.dart';
import '../../../../utils/constants/sizes.dart';

class LoginScreen extends StatelessWidget {
const LoginScreen({super.key});

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
                      decoration: const InputDecoration(prefixIcon: Icon(Icons.password), labelText: "Password",
                      ),
                    ),
                
                    const SizedBox(height: TSizes.spaceBtwItemsInputFields / 2),
                
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                          Row(
                            children: [
                              Checkbox(value: true, onChanged: (value){}),
                              const Text("Remember me"),
                            ],
                          ),
                          TextButton(onPressed: (){}, child: const Text("Forgot password?")),
                      ],
                    ),
                    const SizedBox(height: TSizes.sapceBtwSections),
                
                    SizedBox(width: double.infinity, child: ElevatedButton(onPressed: (){}, child: const Text("Sign in"))),
                    const SizedBox(height: TSizes.spaceBtwItems),
                    SizedBox(width: double.infinity, child: OutlinedButton(onPressed: (){Navigator.push(context, MaterialPageRoute(builder: (constext) => const Registration()));}, child: const Text("Create an account"))),
                
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