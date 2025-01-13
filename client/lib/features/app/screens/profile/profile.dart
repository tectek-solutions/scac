import 'package:flutter/material.dart';
import 'package:client/features/services/api.service.dart';

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
    _apiService.fetchUserProfile().then((value) {
      setState(() {
        _isLoading = false;
        if (value is Map<String, dynamic>) {
          _userProfile = value;
        } else {
          _hasError = true;
        }
      });
    });
  }

  void _logout() {
    // Implement your logout functionality here
    print('User logged out');
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Profile Page'),
        centerTitle: true,
        automaticallyImplyLeading: false,
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Center(
          child: _isLoading
              ? const CircularProgressIndicator()
              : _hasError
                  ? const Text('Error loading profile')
                  : Column(
                      crossAxisAlignment: CrossAxisAlignment.center,
                      children: [
                        CircleAvatar(
                          radius: 50,
                          backgroundImage: NetworkImage(_userProfile?['profile_image_url'] ?? 'https://example.com/profile.jpg'),
                        ),
                        const SizedBox(height: 16),
                        Text(
                          _userProfile?['name'] ?? 'John Doe',
                          style: const TextStyle(
                            fontSize: 24,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          _userProfile?['email'] ?? 'Email here',
                          style: const TextStyle(
                            fontSize: 16,
                            color: Colors.grey,
                          ),
                        ),
                        const SizedBox(height: 16),
                        const Text(
                          'Add your bio here',
                          textAlign: TextAlign.center,
                        ),
                        const SizedBox(height: 30),
                        SizedBox(
                          width: 150,
                          height: 60,
                          child: ElevatedButton(
                            onPressed: _logout,
                            child: const Text('Logout'),
                          ),
                        ),
                      ],
                    ),
        ),
      ),
    );
  }
}