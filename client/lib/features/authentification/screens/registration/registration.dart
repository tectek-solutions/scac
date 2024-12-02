import 'package:client/common/style/spacing_styles.dart';
import 'package:client/features/authentification/screens/login/login.dart';
import 'package:flutter/material.dart';
import '../../../../utils/constants/helper_functions.dart';
import 'package:client/utils/constants/image_strings.dart';
import '../../../../utils/constants/sizes.dart';
import 'package:client/utils/constants/text_string.dart';

class Registration extends StatelessWidget {
  const Registration({super.key});

  @override
  Widget build(BuildContext context) {
    final dark = THelperFunctions.isDarkMode(context);
    return Scaffold(
      body: Center(
        child: Padding(
          padding: TSpacingStyles.defaultPaddingWithAppBarHeight,
          child: ConstrainedBox(
            constraints: BoxConstraints(maxWidth: 800),
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
                      const SizedBox(height: TSizes.sm),
                      Text(
                        TText.registrationSubtitle,
                        style: Theme.of(context).textTheme.bodyMedium,
                      ),
                    ],
                  ),
                  
                  Form(
                    child: Padding(
                      padding: const EdgeInsets.symmetric(
                          vertical: TSizes.sapceBtwSections),
                      child: Column(
                        children: [
                          TextFormField(
                            decoration: const InputDecoration(
                              prefixIcon: Icon(Icons.person),
                              labelText: "Name",
                            ),
                          ),
                          const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                          TextFormField(
                            decoration: const InputDecoration(
                              prefixIcon: Icon(Icons.person),
                              labelText: "First Name",
                            ),
                          ),
                          const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                          TextFormField(
                            decoration: const InputDecoration(
                              prefixIcon: Icon(Icons.email),
                              labelText: "Email",
                            ),
                          ),
                          const SizedBox(height: TSizes.spaceBtwItemsInputFields),
                          TextFormField(
                            decoration: const InputDecoration(
                              prefixIcon: Icon(Icons.password),
                              labelText: "Password",
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),

                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Flexible(child: Divider(color: dark ? Colors.grey.shade700 : Colors.grey.shade300, thickness: 1.5, indent: 60, endIndent: 5)),
                      const Text("Already have an account ?", style: TextStyle(color: Colors.grey)),
                      Flexible(child: Divider(color: dark ? Colors.grey.shade700 : Colors.grey.shade300, thickness: 1.5, indent: 5, endIndent: 60)),
                    ],
                  ),

                  const SizedBox(height: TSizes.sapceBtwSections),

                  Padding(
                    padding: const EdgeInsets.all(8.0),
                    child: LayoutBuilder(
                      builder: (context, constraints) {
                        double buttonWidth = constraints.maxWidth > 600 ? constraints.maxWidth / 3 : constraints.maxWidth / 2.5;

                        return Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Container(
                          width: buttonWidth,
                          child: OutlinedButton(
                            onPressed: () {
                              Navigator.push(context, MaterialPageRoute(builder: (context) => const LoginScreen()));
                            },
                            style: ElevatedButton.styleFrom(
                              side: const BorderSide(
                                color: Colors.blue,
                              ),
                              backgroundColor: Colors.blue,
                            ),
                            child: const Text("Go back"),
                          ),
                        ),
                        const SizedBox(width: TSizes.spaceBtwItems),
                        Container(
                          width: buttonWidth,
                          child: OutlinedButton(
                            onPressed: () {},
                            child: const Text("Sign up"),
                          ),
                        ),
                      ],
                    );
                      },
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
}      
