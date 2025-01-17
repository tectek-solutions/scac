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

  final List<Map<String, String>> data = [
      {"test1": "Enter mail"},
      {"test2": "Workflow"},
      {"test3": "Reaction"},
      {"test4": "Action"},

    ];

  List<Map<String, String>> reactionData = [
    {"reaction1": "Choose trigger"},
    {"reaction2": "Define logic"},
    {"reaction3": "Output action"},
     {"reaction1": "Choose trigger"},
        {"reaction2": "Define logic"},
        {"reaction3": "Output action"},
         {"reaction1": "Choose trigger"},
            {"reaction2": "Define logic"},
            {"reaction3": "Output action"},
             {"reaction1": "Choose trigger"},
                {"reaction2": "Define logic"},
                {"reaction3": "Output action"},
  ];

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
  
    for (var entry in reactionData) {
      final key = entry.keys.first;
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

  void saveData() {
    // Construire un nouveau JSON avec les données saisies
    List<Map<String, String>> updatedData = data.map((entry) {
      final key = entry.keys.first;
      return {key: controllers[key]!.text};
    }).toList();

    List<Map<String, String>> updatedReactionData = reactionData.map((entry) {
      final key = entry.keys.first;
      return {key: reactionControllers[key]!.text};
    }).toList();

    // Afficher ou utiliser les nouvelles données
    print(updatedData);
  }

  void saveReactionData() {

    List<Map<String, String>> updatedReactionData = reactionData.map((entry) {
      final key = entry.keys.first;
      return {key: reactionControllers[key]!.text};
    }).toList();

    print(updatedReactionData);
  }

  void saveWorkflowData() {
    // Construire un nouveau JSON avec les données des menus déroulants
    Map<String, dynamic> workflowData = {
      "values": selectedValues.values.toList(), // Extraire les valeurs sélectionnées
      "parameters": {
        "url": controllers['url']!.text, // Vous pouvez ajouter des contrôleurs spécifiques si nécessaires
        "content": controllers['content']!.text, // Exemple avec des contrôleurs pour d'autres entrées
      },
    };
  
    print(workflowData); // Afficher ou traiter le JSON généré
  }
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
                  MaterialPageRoute(builder: (context) => const ServicePage()),
                );
                if (result != null && result['action'] != null) {
                  setState(() {
                    resultAction = result['action'];
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
                              resultTitleAction,
                              style: const TextStyle(
                                fontSize: 18.0,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                            const SizedBox(height: 8.0),
                            Text(
                              resultDescriptionAction,
                              style: TextStyle(
                                fontSize: 14.0,
                                color: boxColor,
                              ),
                            ),
                            const SizedBox(height: 8.0),
                            Row(
                              children: [
                                Text(
                                  'Action: ',
                                  style: TextStyle(
                                    fontSize: 14.0,
                                    color: boxColor,
                                  ),
                                ),
                                Text(
                                  resultAction,
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
                  MaterialPageRoute(builder: (context) => const ReactionPage()),
                );
                if (result != null && result['action'] != null) {
                  setState(() {
                    resultReaction = result['action'];
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
                                color: boxColor,
                              ),
                            ),
                            const SizedBox(height: 8.0),
                            Row(
                              children: [
                                Text(
                                  'Reaction: ',
                                  style: TextStyle(
                                    fontSize: 14.0,
                                    color: boxColor,
                                  ),
                                ),
                                Text(
                                  resultReaction,
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
            const SizedBox(height: 20.0),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceEvenly, // Espace égal entre les cartes
              children: [
                // Card pour Action
                Card(
                  elevation: 4.0,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(12.0),
                  ),
                  child: Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: SizedBox(
                      height: 300.0, // Même hauteur pour les deux cartes
                      width: 400.0, // Même largeur pour les deux cartes
                      child: SingleChildScrollView(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            const Text(
                              'Action',
                              style: TextStyle(
                                fontSize: 18.0,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                            const SizedBox(height: 20.0),
                            ...data.map((entry) {
                              final key = entry.keys.first;
                              final value = entry.values.first;
                              return Padding(
                                padding: const EdgeInsets.only(bottom: 20.0),
                                child: Row(
                                  mainAxisAlignment: MainAxisAlignment.center,
                                  crossAxisAlignment: CrossAxisAlignment.center,
                                  children: [
                                    Text(
                                      key,
                                      style: const TextStyle(
                                        fontSize: 20.0,
                                        fontWeight: FontWeight.bold,
                                      ),
                                    ),
                                    const SizedBox(width: 28.0),
                                    SizedBox(
                                      width: 200.0,
                                      child: TextField(
                                        controller: controllers[key], // Lier le contrôleur au champ
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
                            const SizedBox(height: 20.0), // Espacement avant le bouton
                            Center(
                              child: ElevatedButton(
                                onPressed: saveData, // Appel de la fonction pour sauvegarder les données
                                child: const Text('Sauvegarder et afficher le JSON'),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),

            
                // Card pour Reaction
                Card(
                  elevation: 4.0,
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(12.0),
                  ),
                  child: Padding(
                    padding: const EdgeInsets.all(16.0),
                    child: SizedBox(
                      height: 300.0, // Même hauteur pour les deux cartes
                      width: 400.0, // Même largeur pour les deux cartes
                      child: SingleChildScrollView(
                        child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            const Text(
                              'Reaction',
                              style: TextStyle(
                                fontSize: 18.0,
                                fontWeight: FontWeight.bold,
                              ),
                            ),
                            const SizedBox(height: 20.0),
                            ...reactionData.map((entry) {
                              final key = entry.keys.first;
                              final value = entry.values.first;
                              return Padding(
                                padding: const EdgeInsets.only(bottom: 20.0),
                                child: Row(
                                  mainAxisAlignment: MainAxisAlignment.center,
                                  crossAxisAlignment: CrossAxisAlignment.center,
                                  children: [
                                    Text(
                                      key,
                                      style: const TextStyle(
                                        fontSize: 20.0,
                                        fontWeight: FontWeight.bold,
                                      ),
                                    ),
                                    const SizedBox(width: 28.0),
                                    SizedBox(
                                      width: 200.0,
                                      child: TextField(
                                        controller: reactionControllers[key], // Lier le contrôleur au champ
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
                            const SizedBox(height: 20.0), // Espacement avant le bouton
                            Center(
                              child: ElevatedButton(
                                onPressed: saveReactionData, // Appel de la fonction pour sauvegarder les données
                                child: const Text('Sauvegarder et afficher le JSON'),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
              ],
            ),
            
            const SizedBox(height: 10.0),
            TextButton(
              onPressed: () {
                // Ajoutez les fonctionnalités pour le bouton ici
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
