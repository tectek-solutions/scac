import 'package:client/features/area/screens/reaction-page/reaction-page.dart';
import 'package:client/features/area/screens/service-page/services-page.dart';
import 'package:flutter/material.dart';

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

  Map<String, String> actions = {};
  Map<String, String> reactions = {};

  final List<Map<String, String>> data = [
      {"test1": "Enter mail"},
      {"test2": "Workflow"},
      {"test3": "Reaction"},
      {"test4": "Action"},

    ];

  Map<String, String> reactionData = {};
  Map<String, TextEditingController> controllers = {};
  

  final List<String> availableValues = ["Option 1", "Option 2", "Option 3"];
  String? selectedValue;

  Map<String, String?> selectedValues = {}; // Stocke les sélections indépendantes
  Map<String, TextEditingController> reactionControllers = {};

  final Color boxColor = Colors.grey[700]!; // Définissez une couleur commune

  @override
  void initState() {
    super.initState();
    // Initialiser chaque clé avec une valeur vide pour permettre à l'utilisateur de rentrer directement du texte
    data.forEach((entry) {
      final key = entry.keys.first;
      selectedValues[key] = availableValues.first; // Vous pouvez mettre une valeur par défaut pour la sélection
    });
  
    for (var entry in data) {
      final key = entry.keys.first;
      controllers[key] = TextEditingController(text: ""); // Initialisez avec une chaîne vide
    }
  
    for (var entry in reactionData.entries) {
      final key = entry.key;
      reactionControllers[key] = TextEditingController(text: ""); // Initialisez avec une chaîne vide
    }
  }
  @override
  void dispose() {
    // Nettoyer les contrôleurs pour éviter les fuites de mémoire
    controllers.forEach((key, controller) => controller.dispose());
    reactionControllers.forEach((key, controller) => controller.dispose());
    super.dispose();
  }

  Widget build(BuildContext context) {
    final screenWidth = MediaQuery.of(context).size.width;

    // Vérification si l'affichage est mobile
    final isMobile = screenWidth < 600;

    reactions.removeWhere((key, value) => key == 'value');

    return Scaffold(
      appBar: AppBar(
        title: const Text('Create Page'),
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

              // Carte d'Action
              GestureDetector(
                onTap: () async {
                  final result = await Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => const ServicePage()),
                  );
                  if (result != null && result['action'] != null) {
                    setState(() {
                      actions = result['action'][0];
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
                  actionValue: resultAction,
                ),
              ),

              const SizedBox(height: 10.0),
              const Icon(Icons.add, size: 30.0),
              const SizedBox(height: 10.0),

              // Carte de Réaction
              GestureDetector(
                onTap: () async {
                  final result = await Navigator.push(
                    context,
                    MaterialPageRoute(
                        builder: (context) => const ReactionPage()),
                  );
                  if (result != null && result['reaction'] != null) {
                    setState(() {
                      reactions = result['reaction'][0];
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
                  actionValue: resultReaction,
                ),
              ),

              const SizedBox(height: 20.0),

              // Cards réactives pour Action et Reaction
              isMobile
                  ? Column(
                      children: [
                        _buildActionReactionCard(
                          title: 'Action',
                          data: actions,
                          controllers: controllers,
                        ),
                        const SizedBox(height: 20.0),
                        _buildActionReactionCard(
                          title: 'Reaction',
                          data: reactions,
                          controllers: reactionControllers,
                        ),
                      ],
                    )
                  : Row(
                      mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                      children: [
                        Flexible(
                          child: _buildActionReactionCard(
                            title: 'Action',
                            data: actions,
                            controllers: controllers,
                          ),
                        ),
                        const SizedBox(width: 16.0),
                        Flexible(
                          child: _buildActionReactionCard(
                            title: 'Reaction',
                            data: reactions,
                            controllers: reactionControllers,
                          ),
                        ),
                      ],
                    ),

              const SizedBox(height: 10.0),
              TextButton(
                onPressed: () {
                  // Fonctionnalités pour le bouton Continue
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

  // Widget générique pour les options (Action et Reaction)
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

  // Widget générique pour les cartes Action et Reaction
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
            const SizedBox(height: 20.0),
            ...data.entries.map((entry) {
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
