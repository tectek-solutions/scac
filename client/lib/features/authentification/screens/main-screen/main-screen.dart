import 'package:client/common/style/spacing_styles.dart';
import 'package:client/features/authentification/screens/login/login.dart';
import 'package:client/utils/constants/colors.dart';
import 'package:client/utils/constants/sizes.dart';
import 'package:flutter/material.dart';


class MainScreen extends StatefulWidget {
  const MainScreen({super.key});

  @override
  // ignore: library_private_types_in_public_api
  _MainScreenState createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {

  bool _isMenuOpen = false;
  Duration duration = const Duration(milliseconds: 200);
  late double screenWidth, screenHeight;

  @override
  Widget build(BuildContext context) {
    Size size = MediaQuery.of(context).size;
    screenWidth = size.width;
    screenHeight = size.height;
    return Scaffold(
      backgroundColor: TColors.backgroundColor,
      body: Stack(
        children: <Widget>[
          menu(context),
          dashboard(context),
        ],
      ),
    );
  }

  Widget menu (constext) {
    return Padding(
      padding: const EdgeInsets.only(left: 0, top: TSizes.appBarHeight),
      child: Container(
        color: TColors.darkColor,
        child: Align(
          alignment: Alignment.topLeft,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.start,
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              const ListTile(
                leading: CircleAvatar(
                  backgroundColor: Colors.white24,
                  radius: 20.00,
                  child: Icon(Icons.person, color: Colors.white,),
                ),
                title: Text("User", style: TextStyle(color: Colors.white, fontSize: TSizes.fontSizeLg),),
                subtitle: Text("email@exmple.com", style: TextStyle(color: Colors.white, fontSize: TSizes.fontSizeSm),),
              ),
              const SizedBox(height: TSizes.spaceBtwItems),
              const ListTile(
                leading: Icon(Icons.dashboard, color: Colors.white,),
                title: Text("Dashboard", style: TextStyle(color: Colors.white, fontSize: TSizes.fontSizeMd),),
              ),
              const ListTile(
                leading: Icon(Icons.person, color: Colors.white,),
                title: Text("Profile", style: TextStyle(color: Colors.white, fontSize: TSizes.fontSizeMd),),
              ),
              const ListTile(
                leading: Icon(Icons.settings, color: Colors.white,),
                title: Text("Settings", style: TextStyle(color: Colors.white, fontSize: TSizes.fontSizeMd),),
              ),
              ListTile(
                onTap: () {
                  Navigator.push(context, MaterialPageRoute(builder: (context) => const LoginScreen()));
                },
                leading: const Icon(Icons.logout, color: Colors.white,),
                title: const Text("Logout", style: TextStyle(color: Colors.white, fontSize: TSizes.fontSizeMd),),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget dashboard (context) {
    return AnimatedPositioned(
      duration: duration,
      top: 0,
      bottom: 0,
      left: _isMenuOpen ? 0 : 0.6 * screenWidth,
      right: _isMenuOpen ? 0 : -0.4 * screenWidth,
      child: Material(
        elevation: 8,
        child: Container(
          padding: TSpacingStyles.defaultPaddingWithAppBarHeight,
          child: Column(
            children: <Widget>[
              Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                mainAxisSize: MainAxisSize.max,
                children: <Widget>[
                  InkWell(child: const Icon(Icons.menu), onTap: (){
                    setState(() {
                      _isMenuOpen = !_isMenuOpen;
                    });
                  },),
                  const Text("Dashboard", style: TextStyle(fontSize: TSizes.fontSizeLg),),
                  const Icon(Icons.settings),
                ],
              ),
            ],
          ),
        )
      ),
    );
  }
}