import 'package:client/features/authentification/screens/reaction-page/reaction-page.dart';
import 'package:client/features/authentification/screens/service-page/services-page.dart';
import 'package:flutter/material.dart';

class CreatePage extends StatefulWidget {
  const CreatePage({super.key});

  @override
  // ignore: library_private_types_in_public_api
  _CreatePageState createState() => _CreatePageState();
}

class _CreatePageState extends State<CreatePage> {
  String resultTitleService = 'Services';
  String resultDescriptionService = 'Go to Services Page';
  String resultActionService = 'No action selected';

  String resultTitleReaction = 'Reactions';
  String resultDescriptionReaction = 'Go to Reaction Page';
  String resultActionReaction = 'No action selected';

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Create Page'),
        centerTitle: true,
        automaticallyImplyLeading: false,
      ),
      body: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(
          children: [
            const Text(
              'Select an option below to proceed:',
              style: TextStyle(
                fontSize: 24.0,
                fontWeight: FontWeight.bold,
              ),
            ),
            const SizedBox(height: 20.0),
            GestureDetector(
              onTap: () async {
                final result = await Navigator.push(
                  context,
                  MaterialPageRoute(builder: (context) => ServicePage()),
                );
                if (result != null && result['action'] != null) {
                  setState(() {
                    resultActionService = result['action'];
                  });
                  print('Data received from Widget B: $result');
                } else {
                  print('No data received');
                }
              },
              child: Card(
                elevation: 4.0,
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12.0),
                ),
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Row(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      const Icon(Icons.build, size: 30.0),
                      const SizedBox(width: 16.0),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              resultTitleService,
                              style: const TextStyle(
                                fontSize: 18.0,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                            const SizedBox(height: 8.0),
                            Text(
                              resultDescriptionService,
                              style: TextStyle(
                                fontSize: 14.0,
                                color: Colors.grey[700]
                              ),
                            ),
                            const SizedBox(height: 8.0),
                            Row(
                              children: [
                                Text(
                                  'Action: ',
                                  style: TextStyle(
                                    fontSize: 14.0,
                                    color: Colors.grey[700],
                                  ),
                                ),
                                Text(
                                  resultActionService,
                                  style: const TextStyle(
                                    fontSize: 14.0,
                                    color: Colors.blue,
                                  ),
                                ),
                              ],
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
            const SizedBox(height: 10.0),
            const Icon(Icons.add, size: 30.0),
            const SizedBox(height: 10.0),
            GestureDetector(
              onTap: () async {
                final result = await Navigator.push(
                  context,
                  MaterialPageRoute(builder: (context) => ReactionPage()),
                );
                if (result != null && result['title'] != null && result['description'] != null && result['action'] != null) {
                  setState(() {
                    resultTitleReaction = result['title'];
                    resultDescriptionReaction = result['description'];
                    resultActionReaction = result['action'];
                  });
                  print('Data received from Widget B: $result');
                } else {
                  print('No data received');
                }
              },
              child: Card(
                elevation: 4.0,
                shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(12.0),
                ),
                child: Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Row(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [
                      const Icon(Icons.new_releases, size: 30.0),
                      const SizedBox(width: 16.0),
                      Expanded(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              resultTitleReaction,
                              style: const TextStyle(
                                fontSize: 18.0,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                            const SizedBox(height: 8.0),
                            Text(
                              resultDescriptionReaction,
                              style: TextStyle(
                                  fontSize: 14.0,
                                  color: Colors.grey[700]
                              ),
                            ),
                            const SizedBox(height: 8.0),
                            Row(
                              children: [
                                Text(
                                  'Action: ',
                                  style: TextStyle(
                                    fontSize: 14.0,
                                    color: Colors.grey[700],
                                  ),
                                ),
                                Text(
                                  resultActionReaction,
                                  style: const TextStyle(
                                    fontSize: 14.0,
                                    color: Colors.blue,
                                  ),
                                ),
                              ],
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
            const SizedBox(height: 10.0),
            TextButton(
              onPressed: () {
                //La tu ajoute les fonctionalités pour quand tu clique sur le bouton
              },
              child: const Text(
                'Continue',
                style: TextStyle(fontSize: 16.0),
              ),
            ),
          ],
        ),
      ),
    );
  }
}