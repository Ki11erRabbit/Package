#include <stdio.h>
#include <stdlib.h>
#include <string.h>


int parseCommandLine (int argc, char *argv[], char command[], char *arguments) {
    char temp[20];
    strcpy(temp, argv[1]);
    if (argc < 2) {
        command = "initialize";
        return 0;
    }
    else if (strcmp("bypass", temp)) {
        command = temp;
        for (unsigned int i = 2; i <= argc; i++) {
            temp = "";
            strcpy(temp, argv[i]);
            arguments += temp;
            arguments += " ";
        }
        return 0;
    }
    else if (strcmp("install", temp) || strcmp("remove", temp)) || strcmp("search", temp) {
        command = temp;
        for (unsigned int i = 2; i <= argc; i++) {
            temp = "";
            strcpy(temp, argv[i]);
            arguments += temp;
            arguments += " ";
        }
        return 0;
    }

    return 1;
    if (argc > 2) { // package - command
        if (strcmp("-", arg[1])) {
            strcmp(option,argv[1]);
            //TODO: if argv[2] compared to Install, update, remove, etc then set argv[2] to command otherwise set everything else to arguments
            //TODO: change to make command first, will make much easier

            strcpy(command, argv[2]);
            //TODO: Write loop to copy to end of line
        }
        else if
    }
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
