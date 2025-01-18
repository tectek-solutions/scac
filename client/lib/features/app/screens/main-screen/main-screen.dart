import 'package:client/common/style/spacing_styles.dart';
import 'package:client/features/app/screens/create-page/create-page.dart';
import 'package:client/features/area/screens/service-page/services-page.dart';
import 'package:client/features/app/screens/explore-page/explore-page.dart';
import 'package:client/features/app/screens/home-page/home-page.dart';
import 'package:client/features/authentification/screens/login/login.dart';
import 'package:client/features/app/screens/profile/profile.dart';
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

  int currentPage = 0;
  final List<Widget> pages = [
    ClickableCardScreen(),
    // const ExplorePage(),
    const CreatePage(),
    const Profile(),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: pages[currentPage],
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: currentPage,
        onTap: (value) {
          setState(() {
            currentPage = value;
          });
        },
        items: const [
          BottomNavigationBarItem(
            icon: Icon(Icons.workspaces_filled),
            label: 'My Workflows',
            backgroundColor: TColors.lightColor,
          ),
          // BottomNavigationBarItem(
          //   icon: Icon(Icons.search),
          //   label: 'Search',
          //   backgroundColor: TColors.lightColor,
          // ),
          BottomNavigationBarItem(
            icon: Icon(Icons.add),
            label: 'Create Workflow',
            backgroundColor: TColors.lightColor,
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.person),
            label: 'Profile',
            backgroundColor: TColors.lightColor,
          ),
        ],
      ),
    );
  }
}