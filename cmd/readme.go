/*
Copyright © 2021 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"fmt"
	"io"
	"net/http"
	"strings"

	"github.com/charmbracelet/glamour"
	"github.com/spf13/cobra"
)

// readmeCmd represents the readme command
var readmeCmd = &cobra.Command{
	Use:   "readme",
	Short: "Prints readme for a repository entered",
	Run: func(cmd *cobra.Command, args []string) {
		if len(args) == 0 {
			panic("readme requires repo url")
		}

		resp, err := http.Get("https://raw.githubusercontent.com/" + args[0] + "/master/README.md")

		if err != nil {
			panic(err)
		}
		defer resp.Body.Close()

		buf := new(strings.Builder)
		_, err = io.Copy(buf, resp.Body)
		if err != nil {
			panic(err)
		}

		out, err := glamour.Render(buf.String(), "dark")
		if err != nil {
			panic(err)
		}
		fmt.Print(out)
	},
}

func init() {
	rootCmd.AddCommand(readmeCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// readmeCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// readmeCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
