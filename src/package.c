#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void setArgs (int argc, char *argv[], unsigned int start, char *arguments) {
    char temp[20];
    for (unsigned int i = start; i <= argc; i++) {
            temp = "";
            strcpy(temp, argv[i]);
            arguments += temp;
            arguments += " ";
        }

}

void setOptions (int argc, char *argv[], char *option[]) {

}

int parseCommandLine (int argc, char *argv[], char command[], char *option[], char *arguments) {
    char temp[20];
    strcpy(temp, argv[1]);
    if (argc < 2) {
        command = "initialize";
        return 0;
    }
    else if (strcmp("bypass", temp)) {//for directing passing arguments into the package manager
        command = temp;

        //setArgs(argc, &argv, 2, &arguments);

        for (unsigned int i = 2; i <= argc; i++) {
            temp = "";
            strcpy(temp, argv[i]);
            arguments += temp;
            arguments += " ";
        }
        return 0;
    }
    else if (strcmp("use", temp)) {//for selecting the package manager to use

    }
    else if (strcmp("install", temp) || strcmp("remove", temp)) || strcmp("search", temp) {
        command = temp;
        //setArgs(argc, &argv, 2, &arguments);

        for (unsigned int i = 2; i <= argc; i++) {
            temp = "";
            strcpy(temp, argv[i]);
            arguments += temp;
            arguments += " ";
        }
        return 0;
    }

    return 1;
}


int main(int argc, char *argv[]) {
    char command[20];
    char option[10];
    char arguments[150]; //TODO: make this more dynamic

    strcpy(option,argv[1]);
    strcpy(command, argv[2]);

    printf ("%s\n", option);
    printf ("%s\n", command);


    printf("Program name %s\n", argv[0]);

   if( argc == 3 ) {
      printf("The argument supplied is %s\n", argv[2]);
   }
   else if( argc > 3 ) {
      printf("Too many arguments supplied.\n");
   }
   else {
      printf("One argument expected.\n");
   }


  /*int grayscale = FALSE;
  unsigned fileSizeInBytes = 0;
  unsigned char* bmpFileAsBytes = NULL;
  FILE *stream = NULL;

  stream = parseCommandLine(argc, argv, &grayscale);
  fileSizeInBytes = getFileSizeInBytes(stream);


  bmpFileAsBytes = (unsigned char *)malloc(fileSizeInBytes);
  if (bmpFileAsBytes == NULL) {
    exit(MALLOC_ERROR);
  }
  getBmpFileAsBytes(bmpFileAsBytes, fileSizeInBytes, stream);

  parseHeaderAndApplyFilter(bmpFileAsBytes, grayscale);



  free(bmpFileAsBytes);*/
  return 0;
}
