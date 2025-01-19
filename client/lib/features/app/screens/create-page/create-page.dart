import 'package:client/features/area/screens/reaction-page/reaction-page.dart';
import 'package:client/features/area/screens/service-page/services-page.dart';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';

class CreatePage extends StatefulWidget {
  const CreatePage({super.key});

  @override
  // ignore: library_private_types_in_public_api
  _CreatePageState createState() => _CreatePageState();
}

class _CreatePageState extends State<CreatePage> {
  String resultTitleAction = 'Actions';
  String resultDescriptionAction = 'Go to Action Page';
  String resultAction = 'No action selected';

  String resultTitleReaction = 'Reactions';
  String resultDescriptionReaction = 'Go to Reaction Page';
  String resultReaction = 'No reaction selected';

  static Map<String, String> actions = {};
  static Map<String, String> actionCleaned = {};
  static Map<String, String> reactions = {};
  static Map<String, String> reactionCleaned = {};

  bool isActionSelected = false;
  bool isReactionSelected = false;

  Map<String, TextEditingController> controllers = {};
  Map<String, TextEditingController> reactionControllers = {};

  final Color boxColor = Colors.grey[700]!;

  @override
  void initState() {
    super.initState();
    _initializeControllers(actionCleaned, controllers);
    _initializeControllers(reactionCleaned, reactionControllers);
  }

  void _initializeControllers(Map<String, String> data, Map<String, TextEditingController> controllers) {
    data.forEach((key, value) {
      controllers[key] = TextEditingController(text: value);
    });
  }

  @override
  void dispose() {
    controllers.forEach((key, controller) => controller.dispose());
    reactionControllers.forEach((key, controller) => controller.dispose());
    super.dispose();
  }

  void createWorkflow(Map<String, dynamic> actionData, Map<String, dynamic> reactionData) async {
  final actionId = actions['id'];
  final reactionId = reactions['id'];
  print('Action Data Here: $actionData');
  print('Reaction Data: $reactionData');
  // final url = Uri.parse('https://your-api-endpoint.com/create-workflow');
  // final response = await http.post(
  //   url,
  //   headers: <String, String>{
  //     'Content-Type': 'application/json; charset=UTF-8',
  //   },
  //   body: jsonEncode(<String, String>{
  //     // Mettre les bonnes valeurs ici
  //   }),
  // );

  // if (response.statusCode == 200) {
  //   print('Workflow created successfully');
  // } else {
  //   print('Failed to create workflow');
  // }
}

  Widget build(BuildContext context) {
  
    final screenWidth = MediaQuery.of(context).size.width;
    final isMobile = screenWidth < 600;

    for (var entry in actions.entries) {
      final key = entry.key;
      final controller = entry.value;
      actionCleaned[key] = controller;
    }

    for (var entry in reactions.entries) {
      final key = entry.key;
      final controller = entry.value;
      reactionCleaned[key] = controller;
    }

    actionCleaned.removeWhere((key, value) => key == 'value');
    reactionCleaned.removeWhere((key, value) => key == 'value');

    isActionSelected = actionCleaned.isNotEmpty;
    isReactionSelected = reactionCleaned.isNotEmpty;

    _syncControllers(actionCleaned, controllers);
    _syncControllers(reactionCleaned, reactionControllers);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Create a workflow'),
        centerTitle: true,
        automaticallyImplyLeading: false,
      ),
      body: SingleChildScrollView(
        child: Padding(
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
                    MaterialPageRoute(builder: (context) => const ServicePage()),
                  );
                  if (result != null && result['index'] != null && result['action'] != null) {
                    setState(() {
                      var index = result['index'];
                      actions = result['action'][index];
                    });
                    print('Data received from Widget B HERE: $result');
                  } else {
                    print('No data received');
                  }
                },
                child: _buildOptionCard(
                  icon: Icons.build,
                  title: resultTitleAction,
                  description: resultDescriptionAction,
                  actionLabel: 'Action: ',
                  actionValue: actions['value'] ?? resultAction,
                ),
              ),

              const SizedBox(height: 10.0),
              const Icon(Icons.add, size: 30.0),
              const SizedBox(height: 10.0),

              GestureDetector(
                onTap: () async {
                  final result = await Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (context) => ReactionPage(actions)),
                  );
                  if (result != null && result['index'] != null && result['reaction'] != null) {
                    setState(() {
                      reactions = result['reaction'][result['index']];
                    });
                    print('Data received from Widget B: $result');
                  } else {
                    print('No data received');
                  }
                },
                child: _buildOptionCard(
                  icon: Icons.new_releases,
                  title: resultTitleReaction,
                  description: resultDescriptionReaction,
                  actionLabel: 'Reaction: ',
                  actionValue: reactions['value'] ?? resultReaction,
                ),
              ),

              const SizedBox(height: 20.0),

              isMobile
                  ? Column(
                      children: [
                        isActionSelected
                            ? _buildActionReactionCard(
                                title: 'Action',
                                data: actionCleaned,
                                controllers: controllers,
                              )
                            : const SizedBox.shrink(),
                        const SizedBox(height: 20.0),
                        isReactionSelected
                            ? _buildActionReactionCard(
                                title: 'Reaction',
                                data: reactionCleaned,
                                controllers: reactionControllers,
                              )
                            : const SizedBox.shrink(),
                      ],
                    )
                  : Row(
                      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                      children: [
                        isActionSelected
                            ?
                        Flexible(
                          child: _buildActionReactionCard(
                            title: 'Action',
                            data: actionCleaned,
                            controllers: controllers,
                          ),
                        ) : const SizedBox.shrink(),
                        const SizedBox(width: 16.0),
                        isReactionSelected
                            ?
                        Flexible(
                          child: _buildActionReactionCard(
                            title: 'Reaction',
                            data: reactionCleaned,
                            controllers: reactionControllers,
                          ),
                        ) : const SizedBox.shrink(),
                      ],
                    ),

              const SizedBox(height: 10.0),
              TextButton(
                onPressed: () {
                  var actionData = {};
                  if (controllers.isEmpty) {
                    actionData = actions;
                  } else {
                    actionData = controllers.map((key, controller) {
                    return MapEntry(key, controller.text.isNotEmpty ? controller.text : actions['value']);
                  });
                  }
                  var reactionData = reactionControllers.map((key, controller) {
                    return MapEntry(key, controller.text);
                  });
                  print('Action Data: $actionData');
                  print('Reaction Data: $reactionData');
                  // Fonction pour créer le workflow (POST request)
                  createWorkflow(actionData.cast<String, dynamic>(), reactionData.cast<String, dynamic>());
                },
                child: const Text(
                  'Create Workflow',
                  style: TextStyle(fontSize: 16.0),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  void _syncControllers(Map<String, String> data, Map<String, TextEditingController> controllers) {
    data.forEach((key, value) {
      if (controllers.containsKey(key)) {
        controllers[key]!.text = value;
      } else {
        controllers[key] = TextEditingController(text: value);
      }
    });
  }

  Widget _buildOptionCard({
    required IconData icon,
    required String title,
    required String description,
    required String actionLabel,
    required String actionValue,
  }) {
    return Card(
      elevation: 4.0,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12.0),
      ),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Icon(icon, size: 30.0),
            const SizedBox(width: 16.0),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    title,
                    style: const TextStyle(
                      fontSize: 18.0,
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                  const SizedBox(height: 8.0),
                  Text(
                    description,
                    style: TextStyle(
                      fontSize: 14.0,
                      color: boxColor,
                    ),
                  ),
                  const SizedBox(height: 8.0),
                  Row(
                    children: [
                      Text(
                        actionLabel,
                        style: TextStyle(
                          fontSize: 14.0,
                          color: boxColor,
                        ),
                      ),
                      Text(
                        actionValue,
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
    );
  }

  Widget _buildActionReactionCard({
    required String title,
    required Map<String, dynamic> data,
    required Map<String, TextEditingController> controllers,
  }) {
    return Card(
      elevation: 4.0,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12.0),
      ),
      child: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              title,
              style: const TextStyle(
                fontSize: 18.0,
                fontWeight: FontWeight.bold,
              ),
            ),
            data.isEmpty
                ? Center(
                    child: Text(
                        'No data required for this ${title.toLowerCase()}.',
                      style: TextStyle(
                        fontSize: 16.0,
                      ),
                    ),
                  )
                :
            const SizedBox(height: 20.0),
            ...data.entries
                .where((entry) => entry.key != 'value')
                .map((entry) {
              final key = entry.key;
              final value = entry.value;
              return Padding(
                padding: const EdgeInsets.only(bottom: 20.0),
                child: Row(
                  children: [
                    Expanded(
                      flex: 2,
                      child: Text(
                        key,
                        style: const TextStyle(
                          fontSize: 20.0,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ),
                    const SizedBox(width: 16.0),
                    Expanded(
                      flex: 3,
                      child: TextField(
                        controller: controllers[key],
                        decoration: InputDecoration(
                          border: OutlineInputBorder(),
                          hintText: value,
                        ),
                      ),
                    ),
                  ],
                ),
              );
            }).toList(),
          ],
        ),
      ),
    );
  }
}