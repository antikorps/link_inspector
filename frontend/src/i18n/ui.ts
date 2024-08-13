export const languages = {
  es: "Español",
  en: "English",
  fr: "Français",
};

export const defaultLang = "es";

export const ui = {
  es: {
    lang: "Español",
    labelLanguage: "Idioma",
    fileFormLabel: "Selecciona tu archivo para procesar:",
    fileFormExtensionValid:
      "Extensiones válidas: docx, pptx, xlsx, html, txt, pdf",
    fileResetButton: "Borrar",
    fileUploadButton: "Enviar",
    errorTextHeading: "¡Ooops! se ha producido un error",
    resultsNumber: "Enlaces encontrados",
    orderLabel: "Ordenar",
    orderSelectOption1: "Activo",
    orderSelectOption2: "URL",
    orderSelectOption3: "Status Code",
    orderSelectOption4: "Texto",
    exportLabel: "Exportar",
    useSummary: "Funcionamiento y uso",
    useExplication:
      "Link Inspector permite verificar los enlaces del documento que adjuntes en el formulario.",
    useResults: "Los resultados se pueden interpretar de esta forma:",
    useGreen:
      "🟢 el status code recibido es correcto por lo que se considera un enlace verificado.",
    useYellow:
      "🟡 el status code recibido es correcto, pero existe algún aspecto que debería comprobarse (es una redirección, la URL incorporada no es la recomendada, etc.).",
    userRed:
      "🔴 el status code es incorrecto o directamente no ha podido comprobarse el enlace incorporado.",
    useInfoStatusCode:
      "En el siguiente enlace encontrarás más información sobre el significado de los status code",
    useVisitGitHub:
      "Visita el repositorio del proyecto en GitHub para buscar nuevas actualizaciones o si quieres participar de alguna forma.",
  },
  en: {
    lang: "English",
    labelLanguage: "Language",
    fileFormLabel: "Select your file to process:",
    fileFormExtensionValid:
      "Valid extensions: docx, pptx, xlsx, html, txt, pdf",
    fileResetButton: "Clear",
    fileUploadButton: "Submit",
    errorTextHeading: "Oops! An error has occurred",
    resultsNumber: "Links found",
    orderLabel: "Order by",
    orderSelectOption1: "Active",
    orderSelectOption2: "URL",
    orderSelectOption3: "Status Code",
    orderSelectOption4: "Text",
    exportLabel: "Export",
    useSummary: "How it works and usage",
    useExplication:
      "Link Inspector allows you to verify the links of the document you attach in the form.",
    useResults: "The results can be interpreted as follows:",
    useGreen:
      "🟢 the status code received is correct so it is considered a verified link.",
    useYellow:
      "🟡 the status code received is correct, but there is some aspect that should be checked (it is a redirect, the embedded URL is not the recommended one, etc.).",
    userRed:
      "🔴 the status code is incorrect or the embedded link could not be checked directly.",
    useInfoStatusCode:
      "In the following link you will find more information about the meaning of status codes",
    useVisitGitHub:
      "Visit the project repository on GitHub to look for new updates or if you want to participate in any way.",
  },
  fr: {
    lang: "Français",
    labelLanguage: "Langue",
    fileFormLabel: "Sélectionnez votre fichier à traiter:",
    fileFormExtensionValid:
      "Extensions valides: docx, pptx, xlsx, html, txt, pdf",
    fileResetButton: "Effacer",
    fileUploadButton: "Soumettre",
    errorTextHeading: "Oups ! Une erreur s'est produite",
    resultsNumber: "Liens trouvés",
    orderLabel: "Trier par",
    orderSelectOption1: "Actif",
    orderSelectOption2: "URL",
    orderSelectOption3: "Status Code",
    orderSelectOption4: "Texte",
    exportLabel: "Exporter",
    useSummary: "Fonctionnement et utilisation",
    useExplication:
      "Link Inspector vous permet de vérifier les liens du document que vous joignez dans le formulaire.",
    useResults: "Les résultats peuvent être interprétés comme suit :",
    useGreen:
      "🟢 le code de statut reçu est correct, il est donc considéré comme un lien vérifié.",
    useYellow:
      "🟡 le code de statut reçu est correct, mais il y a un aspect qui devrait être vérifié (il s'agit d'une redirection, l'URL intégrée n'est pas celle recommandée, etc.).",
    userRed:
      "🔴 le code de statut est incorrect ou le lien intégré n'a pas pu être vérifié directement.",
    useInfoStatusCode:
      "Dans le lien suivant, vous trouverez plus d'informations sur la signification des codes de statut",
    useVisitGitHub:
      "Visitez le référentiel du projet sur GitHub pour rechercher de nouvelles mises à jour ou si vous souhaitez participer d'une manière ou d'une autre.",
  },
} as const;
