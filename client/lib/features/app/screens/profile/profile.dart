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
      ),
      body: Center(
        child: SingleChildScrollView(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 600), // Pour limiter la largeur sur le web
            child: Padding(
              padding: const EdgeInsets.all(16.0),
              child: Center(
                child: _isLoading
                    ? const CircularProgressIndicator()
                    : _hasError
                        ? const Text('Error loading profile')
                        : Column(
                            mainAxisAlignment: MainAxisAlignment.center,
                            crossAxisAlignment: CrossAxisAlignment.center,
                            children: [
                              CircleAvatar(
                                radius: 50,
                                backgroundImage: NetworkImage(
                                  _userProfile?['profile_image_url'] ??
                                      'https://example.com/profile.jpg',
                                ),
                              ),
                              const SizedBox(height: 16),
                              Text(
                                _userProfile?['name'] ?? 'John Doe',
                                style: const TextStyle(
                                  fontSize: 24,
                                  fontWeight: FontWeight.bold,
                                ),
                                textAlign: TextAlign.center,
                              ),
                              const SizedBox(height: 8),
                              Text(
                                _userProfile?['email'] ?? 'Email here',
                                style: const TextStyle(
                                  fontSize: 16,
                                  color: Colors.grey,
                                ),
                                textAlign: TextAlign.center,
                              ),
                              const SizedBox(height: 30),
                              SizedBox(
                                width: 150,
                                height: 60,
                                child: ElevatedButton(
                                  onPressed: () async {
                                    await _apiService.signOut();
                                    Navigator.push(
                                      context,
                                      MaterialPageRoute(
                                        builder: (context) =>
                                            const LoginScreen(),
                                      ),
                                    );
                                  },
                                  child: const Text('Logout'),
                                ),
                              ),
                            ],
                          ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
