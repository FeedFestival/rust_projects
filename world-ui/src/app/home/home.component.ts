import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { ElectronService } from '../core/services';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss'],
})
export class HomeComponent implements OnInit {
  private folderStructure: any;

  constructor(
    private router: Router,
    private electronService: ElectronService,
  ) {}

  ngOnInit(): void {
    this.folderStructure = this.extractFolder(__dirname);

    console.log('this.folderStructure: ', this.folderStructure);
  }

  buildAndDraw(): void {
    let path = this.folderStructure?.path + '\\world\\';
    let batfileName = "build-and-draw.bat";
    this.executeBatfile(path, batfileName);
  }

  loadAndDraw(): void {
    let path = this.folderStructure?.path + '\\world\\';
    let command = "cargo run load draw";
    this.executeCommand(path, command);
  }

  private executeCommand(commandPath: string, command: string): void {
    console.log(`Running command: ${command} in "${commandPath}"`);
    this.executeCommandInFolder(commandPath, command);
  }

  private executeBatfile(commandPath: string, batFileName: string): void {
    console.log(`Running bat file: ${batFileName} in "${commandPath}"`);
    this.executeCommandInFolder(commandPath, batFileName);
  }

  private executeCommandInFolder(commandPath: string, command: string) {
    let execCmd = this.electronService.childProcess.spawn(command, {
      shell: true,
      detached: true,
      cwd: commandPath,
    });
    execCmd.on('close', (close) => {
      console.log(`close -> ${close}`);
    });
    execCmd.on('disconnect', (event: any) => {
      console.log(`disconnect -> ${event}`);
    });
    execCmd.on('error', (data) => {
      console.log(`error -> ${data}`);
    });
    execCmd.on('exit', (event: any) => {
      console.log(`exit -> ${event}`);
    });
    execCmd.on('message', (event: any) => {
      console.log(`message -> ${event}`);
    });
    execCmd.on('spawn', (event: any) => {
      console.log(`spawn -> ${event}`);
    });
  }

  private extractFolder(path: string): any | null {
    const worldUiIndex = path.indexOf('\\world-ui');

    if (worldUiIndex !== -1) {
      const project_path = path.substring(0, worldUiIndex);
      const folder = path.substring(worldUiIndex, worldUiIndex + 9); // Skip "\world-ui"

      return { folder, path: project_path };
    }
    return null;
  }
}
