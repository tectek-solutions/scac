import 'package:flutter/material.dart';
import 'package:client/features/services/api.service.dart';
import 'package:client/features/authentification/screens/login/login.dart';

class Profile extends StatefulWidget {
  const Profile({super.key});

  @override
  State<Profile> createState() => _ProfileState();
}

class _ProfileState extends State<Profile> {
  final ApiAccountService _apiService = ApiAccountService(baseUrl: baseUrlString);
  static const baseUrlString = String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');
  Map<String, dynamic>? _userProfile;
  bool _isLoading = true;
  bool _hasError = false;

  @override
  void initState() {
    super.initState();
    _fetchUserProfile();
  }

  Future<void> _fetchUserProfile() async {
    try {
      final profile = await _apiService.fetchUserProfile();
      setState(() {
        _isLoading = false;
        if (profile is Map<String, dynamic>) {
          _userProfile = profile;
        } else {
          _hasError = true;
        }
      });
    } catch (e) {
      setState(() {
        _isLoading = false;
        _hasError = true;
      });
      await storage.deleteAll();
      print('Error fetching user profile: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Profile Page'),
        centerTitle: true,
        automaticallyImplyLeading: false,
        backgroundColor: Colors.teal, // Modifier la couleur de l'AppBar
      ),
      body: Center(
        child: SingleChildScrollView(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600), // Limiter la largeur sur le web
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: _isLoading
                  ? const Center(child: CircularProgressIndicator())
                  : _hasError
                      ? const Center(child: Text('Error loading profile'))
                      : Column(
                          mainAxisAlignment: MainAxisAlignment.center,
                          crossAxisAlignment: CrossAxisAlignment.center,
                          children: [
                            CircleAvatar(
                              radius: 70,
                              backgroundImage: NetworkImage(
                                _userProfile?['profile_image_url'] ??
                                    'https://example.com/profile.jpg',
                              ),
                            ),
                            const SizedBox(height: 20),
                            Text(
                              _userProfile?['name'] ?? 'John Doe',
                              style: const TextStyle(
                                fontSize: 28,
                                fontWeight: FontWeight.bold,
                                color: Colors.teal,
                              ),
                              textAlign: TextAlign.center,
                            ),
                            const SizedBox(height: 10),
                            Text(
                              _userProfile?['email'] ?? 'Email here',
                              style: const TextStyle(
                                fontSize: 18,
                                color: Colors.grey,
                              ),
                              textAlign: TextAlign.center,
                            ),
                            const SizedBox(height: 30),
                            SizedBox(
                              width: 200,
                              height: 50,
                              child: ElevatedButton(
                                onPressed: () async {
                                  await _apiService.signOut();
                                  Navigator.pushReplacement(
                                    context,
                                    MaterialPageRoute(
                                      builder: (context) =>
                                          const LoginScreen(),
                                    ),
                                  );
                                },
                                style: ElevatedButton.styleFrom(
                                  backgroundColor: Colors.teal,
                                  shape: RoundedRectangleBorder(
                                    borderRadius: BorderRadius.circular(12.0),
                                  ),
                                  textStyle: const TextStyle(
                                    fontSize: 18,
                                    fontWeight: FontWeight.bold,
                                  ),
                                ),
                                child: const Text('Logout'),
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
