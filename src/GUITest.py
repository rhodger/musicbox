from tkinter import *
import subprocess

target = "DEFAULT"

class MenuWindow(Frame):
    def __init__(self, master=None):
        Frame.__init__(self, master)                 
        self.master = master
        self.init_window()

    #Creation of init_window
    def init_window(self):

        # changing the title of our master widget      
        self.master.title("Menu")

        # allowing the widget to take the full space of the root window
        self.pack(fill=BOTH, expand=1)

        # creating a button instance
        j = 0
        for i in subprocess.check_output(["./musicbox", "display", "../../content/Albums.txt"]).decode("ASCII").splitlines():
            option = Button(self, text=i, command= lambda: viewAlbum(i))
            option.pack()

class AlbumWindow(Frame):
    def __init__(self, master=None):
        Frame.__init__(self, master)                 
        self.master = master
        self.init_window()

    #Creation of init_window
    def init_window(self):
        global target

        # changing the title of our master widget      
        self.master.title("GUI")

        # allowing the widget to take the full space of the root window
        self.pack(fill=BOTH, expand=1)

        print(subprocess.check_output(["./musicbox", "find", "../../content/Albums.txt", target]).decode('ASCII'))

        # creating a button instance
        j = 0
        for i in subprocess.check_output(["./musicbox", "find", "../../content/Albums.txt", target]).decode('ASCII').splitlines():
            quitButton = Button(self, text=i)
            quitButton.place(x=0, y=(j*25))
            j += 1

def viewAlbum(album):
    global target
    target = album
    
    root = Tk()
    root.geometry("80x500")
    app = AlbumWindow(root)
    root.mainloop()

root = Tk()

root.geometry("80x500")


app = MenuWindow(root)
root.mainloop()
