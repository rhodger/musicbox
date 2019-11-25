from tkinter import *
import subprocess

class Window(Frame):


    def __init__(self, master=None):
        Frame.__init__(self, master)                 
        self.master = master
        self.init_window()

    #Creation of init_window
    def init_window(self):

        # changing the title of our master widget      
        self.master.title("GUI")

        # allowing the widget to take the full space of the root window
        self.pack(fill=BOTH, expand=1)

        # creating a button instance
        j = 0
        for i in subprocess.check_output(["./musicbox", "display", "../../content/Albums.txt"]).decode('ASCII').splitlines():
            quitButton = Button(self, text="Quit")
            quitButton.place(x=0, y=(j*10))
            j += 1

root = Tk()

#size of the window
root.geometry("400x300")

print(subprocess.check_output(["./musicbox", "find", "../../content/Albums.txt", "borgr"]).decode('ASCII'))

app = Window(root)
root.mainloop()
